mod machine;
mod processor;
mod sic_xe;

use machine::Machine;
use processor::Processor;
use tokio::time::{self, Duration};

use crate::processor::{ProcessorExt, ProcessorHandle};

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures::{StreamExt};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
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

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // test_machine();
    // test_processor();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

// Ratatui
/// The main application which holds the state and logic of the application.
pub struct App {
    running: bool,

    command_buffer: String,
    showing_memory_location: usize,

    processor_ptr: ProcessorHandle,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: false,
            processor_ptr: Processor::new_handle(),
            command_buffer: String::new(),
            showing_memory_location: 0,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;

        let mut events = EventStream::new();
        let mut tick = time::interval(Duration::from_millis(16)); // 60 FPS

        while self.running {
            tokio::select! {
                // UI tick
                _ = tick.tick() => {
                    if let Err(e) = terminal.draw(|frame| self.render(frame)) {
                        return Err(e.into());
                    }
                }

                // Input event
                maybe_evt = events.next() => {
                    match maybe_evt {
                        Some(Ok(evt)) => self.handle_event(evt),
                        Some(Err(e)) => return Err(e.into()),
                        None => {
                            // Event stream ended -> exit
                            self.running = false;
                        }
                    }
                }
            }
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
                Constraint::Percentage(68), // upper row (memory + disasm)
                Constraint::Percentage(32), // lower row
            ])
            .split(main_chunks[0]);

        // UPPER ROW: [ memory ][ disasm ]
        let upper_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), // memory
                Constraint::Percentage(60), // disasm
            ])
            .split(top_chunks[0]);

        // ===== LOWER ROW: [ registers ][ output ][ info ] =====
        let lower_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // registers
                Constraint::Percentage(40), // output
                Constraint::Percentage(30), // info
            ])
            .split(top_chunks[1]);

        // ===== MEMORY PANE =====
        let mut mem_lines = Vec::new();
        let mut line_value = String::new();
        for i in 0..320 {
            let memory_location = self.showing_memory_location + i;

            if i % 16 == 0 {
                if !line_value.is_empty() {
                    mem_lines.push(Line::from(line_value));
                }

                line_value = String::new();
                line_value.push_str(&format!("{:04x}: ", memory_location));
            }
            line_value
                .push_str(&format!("{:0>2x} ", processor.machine.memory.get_byte(memory_location)));
        }
        if !line_value.is_empty() {
            mem_lines.push(Line::from(line_value));
        }

        let mem_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title("Memory")
            .title_style(Style::default().fg(Color::Red));
        let mem_widget = Paragraph::new(mem_lines).block(mem_block);
        frame.render_widget(mem_widget, upper_chunks[0]);

        // ===== DISASSEMBLY PANE =====
        let disasm_lines = vec![Line::from("Nothing to dissasembly")];

        let disasm_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Disassembly")
            .title_style(Style::default().fg(Color::Green));
        let disasm_widget = Paragraph::new(disasm_lines).block(disasm_block);
        frame.render_widget(disasm_widget, upper_chunks[1]);

        // ===== PROCESSOR PANE =====
        let regs_lines = vec![
            Line::from(format!(" A = {:6x}", processor.machine.registers.get_a())),
            Line::from(format!(" X = {:6x}", processor.machine.registers.get_x())),
            Line::from(format!(" L = {:6x}", processor.machine.registers.get_l())),
            Line::from(format!(" B = {:6x}", processor.machine.registers.get_b())),
            Line::from(format!(" S = {:6x}", processor.machine.registers.get_s())),
            Line::from(format!(" T = {:6x}", processor.machine.registers.get_t())),
            Line::from(format!("PC = {:6x}", processor.machine.registers.get_pc())),
            Line::from(format!("SW = {:6x}", processor.machine.registers.get_sw())),
            Line::from(format!("Speed in hz: {}", processor.get_speed())),
        ];

        let regs_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title("Processor")
            .title_style(Style::default().fg(Color::Blue));

        let regs_widget = Paragraph::new(regs_lines).block(regs_block);
        frame.render_widget(regs_widget, lower_chunks[0]);

        // ===== OUTPUT PANE =====
        let output_text = processor.machine.output_text();
        let output_lines: Vec<Line> = if output_text.is_empty() {
            vec![Line::from("No output yet")]
        } else {
            output_text.lines().map(Line::from).collect()
        };

        let output_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title("Output")
            .title_style(Style::default().fg(Color::Yellow));

        let output_widget = Paragraph::new(output_lines).block(output_block);
        frame.render_widget(output_widget, lower_chunks[1]);

        // ===== INFO PANE =====
        let info_lines = vec![
            Line::from("Commands:"),
            Line::from("  q            quit"),
            Line::from("  start        start processor"),
            Line::from("  stop         stop processor"),
            Line::from("  step         one step"),
            Line::from("  load <file>  load program"),
            Line::from("  f <hz>       set speed"),
            Line::from("  mem <addr>   show memory from addr"),
            Line::from(""),
        ];

        let info_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .title("Info")
            .title_style(Style::default().fg(Color::Magenta));

        let info_widget = Paragraph::new(info_lines).block(info_block);
        frame.render_widget(info_widget, lower_chunks[2]);

        // ===== CLI PANE =====
        let prompt_line = Line::from(self.command_buffer.clone());

        let cli_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Input")
            .title_style(Style::default().fg(Color::White));

        let cli_widget = Paragraph::new(prompt_line).block(cli_block);
        frame.render_widget(cli_widget, main_chunks[1]);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_event(&mut self, evt: Event) {
        match evt {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Enter) => {
                let cmd = self.command_buffer.trim().to_string();
                if !cmd.is_empty() {
                    let cmds: Vec<&str> = cmd.split_whitespace().collect();
                    self.execute_command(cmds);
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
    fn execute_command(&mut self, cmd: Vec<&str>) {
        match cmd.as_slice() {
            ["q"] => self.quit(),
            ["step"] => {
                self.processor_ptr.step();
            }
            ["start"] => {
                self.processor_ptr.start();
            }
            ["stop"] => {
                self.processor_ptr.stop();
            }
            ["load", file] => self.processor_ptr.load_file(file),
            ["f", hz] => {
                if let Ok(value) = hz.parse::<i64>() {
                    self.processor_ptr.set_speed(value);
                }
            }
            ["mem", memory_location] => {
                if let Ok(value) = memory_location.parse::<usize>() {
                    self.showing_memory_location = value;
                }
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) { self.running = false; }
}
