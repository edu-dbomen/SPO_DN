mod machine;
mod processor;
mod sic_xe;

use machine::Machine;
use processor::Processor;

use crate::processor::{ProcessorExt, ProcessorHandle};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

fn test_processor() {
    let processor_ptr = Processor::new_handle();
    // processor_ptr.load_file("./tests/arith.obj");
    // processor_ptr.load_file("./tests/horner.obj");
    processor_ptr.load_file("./tests/rec.obj");
    processor_ptr.start();
    loop {}
}

fn test_machine() {
    // write HELLO: to output
    let mut machine = Machine::new();
    machine.get_device(1).write(0x48);
    machine.get_device(1).write(0x45);
    machine.get_device(1).write(0x4C);
    machine.get_device(1).write(0x4C);
    machine.get_device(1).write(0x4F);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x0A);

    // get input
    let _ = machine.get_device(0).read();

    // write HI to output
    machine.memory.set_byte(0xabcd, 0x48);
    machine.memory.set_byte(0xabce, 0x49);
    machine.memory.set_byte(0xabcf, 0x0A);
    let word = machine.memory.get_word(0xabcd);
    for byte in word {
        machine.get_device(1).write(byte.clone());
    }

    // change register A values and write to output
    machine.get_device(1).write(0x41);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x20);
    let val_a = machine.registers.get_a() as u8;
    machine.get_device(1).write(val_a);
    machine.get_device(1).write(0x0A);

    machine.registers.set_a(69);
    machine.get_device(1).write(0x41);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x20);
    let val_a = machine.registers.get_a() as u8;
    machine.get_device(1).write(val_a);
    machine.get_device(1).write(0x0A);
}

fn main() -> color_eyre::Result<()> {
    // test_machine();
    // test_processor();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

// Ratatui
/// The main application which holds the state and logic of the application.
pub struct App {
    /// Is the application running?
    running: bool,
    command_buffer: String,

    processor_ptr: ProcessorHandle,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: false,
            processor_ptr: Processor::new_handle(),
            command_buffer: String::new(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let processor = self.processor_ptr.lock().unwrap();

        let area = frame.area();

        // OUTER LAYOUT: [ top panes ][ CLI ]
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(5),    // top (memory + disasm + regs)
                Constraint::Length(3), // bottom (CLI)
            ])
            .split(area);

        // TOP LAYOUT: [ upper row ][ registers ]
        let top_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70), // upper row (memory + disasm)
                Constraint::Percentage(30), // registers
            ])
            .split(main_chunks[0]);

        // UPPER ROW: [ memory ][ disasm ]
        let upper_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // memory
                Constraint::Percentage(50), // disasm
            ])
            .split(top_chunks[0]);

        // ===== MEMORY PANE =====
        let mut mem_lines = Vec::new();
        let mut line_value = String::new();
        for i in 0..320 {
            if i % 16 == 0 {
                if !line_value.is_empty() {
                    mem_lines.push(Line::from(line_value));
                }

                line_value = String::new();
                line_value.push_str(&format!("{:04x}: ", i));
            }
            line_value.push_str(&format!("{:0>2x} ", processor.machine.memory.get_byte(i)));
        }
        if !line_value.is_empty() {
            mem_lines.push(Line::from(line_value));
        }

        let mem_title = "Memory";
        let mem_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title(mem_title)
            .title_style(Style::default().fg(Color::Red));
        let mem_widget = Paragraph::new(mem_lines).block(mem_block);
        frame.render_widget(mem_widget, upper_chunks[0]);

        // ===== DISASSEMBLY PANE =====
        let disasm_lines = vec![Line::from("Nothing to dissasembly")];

        let disasm_title = "Disassembly";
        let disasm_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(disasm_title)
            .title_style(Style::default().fg(Color::Green));
        let disasm_widget = Paragraph::new(disasm_lines).block(disasm_block);
        frame.render_widget(disasm_widget, upper_chunks[1]);

        // ===== REGISTERS PANE =====
        let regs_lines = vec![
            Line::from(format!("A = {:6x}", processor.machine.registers.get_a())),
            Line::from(format!("X = {:6x}", processor.machine.registers.get_x())),
            Line::from(format!("L = {:6x}", processor.machine.registers.get_l())),
            Line::from(format!("B = {:6x}", processor.machine.registers.get_b())),
            Line::from(format!("S = {:6x}", processor.machine.registers.get_s())),
            Line::from(format!("T = {:6x}", processor.machine.registers.get_t())),
        ];

        let regs_title = "Registers";
        let regs_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(regs_title)
            .title_style(Style::default().fg(Color::Blue));

        let regs_widget = Paragraph::new(regs_lines).block(regs_block);
        frame.render_widget(regs_widget, top_chunks[1]);

        // ===== CLI PANE =====
        let prompt_line = Line::from(self.command_buffer.clone());

        let cli_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(regs_title)
            .title_style(Style::default().fg(Color::Yellow));

        let cli_widget = Paragraph::new(prompt_line).block(cli_block);
        frame.render_widget(cli_widget, main_chunks[1]);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Enter) => {
                let cmd = self.command_buffer.trim().to_string();
                if !cmd.is_empty() {
                    if cmd == "q" {
                        self.quit()
                    }
                    // self.execute_command(&cmd);
                }
                self.command_buffer.clear();
            }
            (_, KeyCode::Backspace) => {
                self.command_buffer.pop();
            }
            (_, KeyCode::Char(c)) => {
                self.command_buffer.push(c);
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) { self.running = false; }
}
