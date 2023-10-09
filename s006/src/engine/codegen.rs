use super::{parser::AST, Instruction};
use crate::helper::safe_add;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum CodeGenError {
    PCOverFlow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for CodeGenError {}

/// コード生成器
#[derive(Default, Debug)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

pub fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.get_code(ast)?;
    Ok(generator.insts)
}

impl Generator {
    fn get_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        self.gen_expr(ast)?;
        self.inc_pc()?;
        self.insts.push(Instruction::Match);
        Ok(())
    }

    // 命令が1つ生成されるとpcの値がインクリメントされ、そのみいれいがinputsにプッシュされる
    // インクリメントにはsafe_add関数を用い、オーバーフローによる不正な動作を検知
    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        // safe_addの第2引数にはオーバーフローが起きた場合のエアラーを返すクロージャを渡す
        // 値を直接渡すのではなくクロージャを渡すことで、Boxによるメモリアロケーションを遅延させることができる
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverFlow)
    }

    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Or(e1, e2) => self.gen_or(e1, e2)?,
            AST::Plus(e) => self.gen_plus(e)?,
            AST::Star(e1) => match &**e1 {
                AST::Star(_) => self.gen_expr(&e1)?,
                AST::Seq(e2) if e2.len() == 1 => {
                    if let Some(e3 @ AST::Star(_)) = e2.get(0) {
                        self.gen_expr(e3)?
                    } else {
                        self.gen_star(e1)?
                    }
                }
                e => self.gen_star(&e)?,
            },
            AST::Question(e) => self.gen_question(e)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }

        Ok(())
    }

    fn gen_plus(&mut self, e: &AST) -> Result<(), CodeGenError> {
        let l1 = self.pc;
        self.gen_expr(e)?;

        self.inc_pc()?;
        let split = Instruction::Split(l1, self.pc);

        self.insts.push(split);

        Ok(())
    }

    fn gen_star(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // L1: split L2, L3
        let l1 = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L2: eのコード
        self.gen_expr(e)?;

        // jump L1
        self.inc_pc();
        self.insts.push(Instruction::Jump(l1));

        // L3の値の設定
        if let Some(Instruction::Split(_, l3)) = self.insts.get_mut(l1) {
            *l3 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailStar)
        }
    }

    fn gen_seq(&mut self, exprs: &[AST]) -> Result<(), CodeGenError> {
        for e in exprs {
            self.gen_expr(e)?;
        }

        Ok(())
    }

    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    /// ```text
    ///     split L1, L2
    /// L1: e1 のコード
    ///   jmp L3
    /// L2: e2のコード
    /// L3:
    /// ```
    fn gen_or(&mut self, e1: &AST, e2: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc();
        // L1 = self.pc L2は仮に0と設定
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L1: e1のコード
        self.gen_expr(e1)?;

        // jmp L3
        let jmp_addr = self.pc;
        self.insts.push(Instruction::Jump(0)); // L3を仮に0と設定

        // L2の値を設定
        self.inc_pc()?;
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr); // 参考書はBox::newしているがコンパイルエラーが出る(?)
        }

        // L2: e2のコード
        self.gen_expr(e2)?;

        // L3の値を設定
        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jmp_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    fn gen_question(&mut self, e: &AST) -> Result<(), CodeGenError> {
        let split_addr = self.pc;
        self.inc_pc();
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        self.gen_expr(e)?;

        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailQuestion)
        }
    }
}
