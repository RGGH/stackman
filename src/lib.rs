pub struct Calculator {
    stack: Vec<i32>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { stack: Vec::new() }
    }

    pub fn push(&mut self, num: i32) {
        self.stack.push(num);
    }

    pub fn add(&mut self) {
        let b = self.stack.pop().expect("Stack underflow");
        let a = self.stack.pop().expect("Stack underflow");
        self.stack.push(a + b);
    }
    pub fn sub(&mut self) {
        let b = self.stack.pop().expect("Stack underflow");
        let a = self.stack.pop().expect("Stack underflow");
        self.stack.push(a - b);
    }

    pub fn equal(&self) -> Option<i32> {
        self.stack.last().cloned()
    }

    pub fn dup(&mut self) {
        if let Some(&top) = self.stack.last() {
            self.stack.push(top);
        } else {
            println!("Stack is empty");
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut calc = Calculator::new();
        calc.push(5);
        assert_eq!(calc.stack, vec![5]);
    }

    #[test]
    fn test_add() {
        let mut calc = Calculator::new();
        calc.push(5);
        calc.push(3);
        calc.add();
        assert_eq!(calc.stack, vec![8]);
    }

    #[test]
    fn test_equal() {
        let mut calc = Calculator::new();
        calc.push(5);
        assert_eq!(calc.equal(), Some(5));
        calc.push(3);
        calc.add();
        assert_eq!(calc.equal(), Some(8));
    }
}
