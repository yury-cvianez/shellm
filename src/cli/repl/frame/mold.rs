const WIDTH: usize = 64; // largura total, bordas incluídas

pub struct Frame {
    title: String,
    lines: Vec<String>,
}

impl Frame {
    pub fn new(title: impl Into<String>) -> Self {
        Frame { title: title.into(), lines: Vec::new() }
    }

    pub fn line(&mut self, s: impl Into<String>) {
        self.lines.push(s.into());
    }

    /// várias linhas de uma vez (ex.: saída de um comando)
    pub fn text(&mut self, s: &str) {
        for l in s.lines() {
            self.lines.push(l.to_string());
        }
    }

    pub fn print(&self) {
        let inner = WIDTH - 4; // borda + espaço de cada lado
        println!("{}", self.top());
        for line in &self.lines {
            for row in wrap(line, inner) {
                let pad = inner - row.chars().count();
                println!("│ {row}{} │", " ".repeat(pad));
            }
        }
        println!("└{}┘", "─".repeat(WIDTH - 2));
    }

    fn top(&self) -> String {
        if self.title.is_empty() {
            return format!("┌{}┐", "─".repeat(WIDTH - 2));
        }
        let title: String = self.title.chars().take(WIDTH - 6).collect();
        let fill = WIDTH - 5 - title.chars().count();
        format!("┌─ {title} {}┐", "─".repeat(fill))
    }
}

/// expande tab, remove caracteres de controle e quebra por largura
fn wrap(line: &str, width: usize) -> Vec<String> {
    let chars: Vec<char> = line
        .chars()
        .flat_map(|c| match c {
            '\t' => vec![' '; 4],
            c if c.is_control() => vec![],
            c => vec![c],
        })
        .collect();

    if chars.is_empty() {
        return vec![String::new()];
    }
    chars.chunks(width).map(|c| c.iter().collect()).collect()
}

pub fn banner() {
    let mut f = Frame::new("");
    f.line("sshelm 🦀");
    f.print();
}