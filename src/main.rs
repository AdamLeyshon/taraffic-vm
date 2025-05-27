mod rgal;
mod shared;
mod tpu;

use crate::shared::{AnalogPin, DigitalPin, Register};
use crate::tpu::create_basic_tpu_config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use strum::IntoEnumIterator;
use tracing::Level;
use tracing_subscriber;
use tracing_subscriber::fmt::format;

fn main() -> Result<(), Box<dyn Error>> {
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::TRACE)
    //     .init();

    // Create app state
    let program = rgal::parse_program(
        r#"
        LDR A, 10
        LDR X, 0x5555
        DPWW X
        ROL X, X, 1
        DEC A
        BEZ 7, A
        JMP 2
        HLT"#,
    )
    .unwrap();

    let mut tpu = create_basic_tpu_config(program);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal, &mut tpu);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    tpu: &mut tpu::TPU,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, tpu.state()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('s') => {
                        tpu.step();
                    }
                    KeyCode::Char(' ') => {
                        tpu.tick();
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, tpu: &tpu::TpuState) {
    // Create main layout with title and content areas
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Content
            ]
            .as_ref(),
        )
        .split(f.size());

    // Title
    let title = Paragraph::new("TPU Simulator - Press Space to tick, S to Step, Q to quit")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, main_chunks[0]);

    // Split content area into left and right columns
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // Left column
                Constraint::Percentage(50), // Right column
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    // Split left column into sections
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25), // CPU Status
                Constraint::Percentage(25), // Registers
                Constraint::Percentage(25), // Network
                Constraint::Percentage(25), // Stack
            ]
            .as_ref(),
        )
        .split(content_chunks[0]);

    // Split right column into sections
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33), // RAM
                Constraint::Percentage(33), // ROM
                Constraint::Percentage(34), // I/O Pins
            ]
            .as_ref(),
        )
        .split(content_chunks[1]);

    // Render each component
    render_cpu_status(f, tpu, left_chunks[0]);
    render_registers(f, tpu, left_chunks[1]);
    render_network(f, tpu, left_chunks[2]);
    render_stack(f, tpu, left_chunks[3]);
    render_ram(f, tpu, right_chunks[0]);
    render_rom(f, tpu, right_chunks[1]);
    render_io_pins(f, tpu, right_chunks[2]);
}

fn render_cpu_status(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let halted = tpu.halted;
    let program_counter = tpu.program_counter;
    let wait_cycles = tpu.execution_state.wait_cycles;
    let text = format!(
        "Program Counter: {:04X}\nWait Cycles: {:04X}\nHalted: {}",
        program_counter, wait_cycles, halted
    );
    let widget =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("TPU Status"));
    f.render_widget(widget, area);
}

fn render_registers(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let mut text = String::new();
    for register in Register::iter() {
        let value = tpu.registers[register as usize];
        text.push_str(&format!("{:2}: {:04X}\n", format!("{:?}", register), value));
    }
    let widget =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Registers"));
    f.render_widget(widget, area);
}

fn render_network(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let network_address = tpu.network_address;
    let incoming_packets = tpu.incoming_packets.len();
    let outgoing_packets = tpu.outgoing_packets.len();

    let text = format!(
        "Network Address: {:04X}\nIncoming Packets: {}\nOutgoing Packets: {}",
        network_address, incoming_packets, outgoing_packets
    );

    let widget =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Network"));
    f.render_widget(widget, area);
}

fn render_stack(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let stack_size = tpu.stack.len();
    let stack_contents = &tpu.stack;

    let mut text = format!("Stack Size: {}\n", stack_size);

    if stack_contents.is_empty() {
        text.push_str("<empty>");
    } else {
        for (i, &value) in stack_contents.iter().enumerate() {
            text.push_str(&format!("{}: {:04X}\n", i, value));
        }
    }

    let widget = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Stack"));
    f.render_widget(widget, area);
}

fn render_ram(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let ram_size = tpu.ram.len();

    let mut text = String::new();

    // Display a portion of RAM (first 32 bytes)
    for i in 0..ram_size {
        if i % 4 == 0 && i > 0 {
            text.push('\n');
        }
        let value = tpu.ram[i];
        text.push_str(&format!("{:04X}: {:04X} ", i, value));
    }

    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("RAM, {} words", ram_size)),
    );
    f.render_widget(widget, area);
}

fn render_rom(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let rom = &tpu.rom;
    let rom_size = rom.len();
    let program_counter = tpu.program_counter;

    let mut text = format!(
        "ROM Size: {}\nProgram Counter: {:04X}\n \n  ADDR  INSTRUCTION\n  ----  ------------\n",
        rom_size, program_counter
    );

    // Display a portion of ROM (first 8 instructions)
    //let display_size = std::cmp::min(8, rom_size);
    for i in 0..rom_size {
        if let Some(instruction) = tpu.rom.get(i) {
            let marker = if i == program_counter { ">" } else { " " };
            text.push_str(&format!("{} {:04X}: {}\n", marker, i, instruction));
        }
    }

    let widget = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("ROM"));
    f.render_widget(widget, area);
}

fn render_io_pins(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50), // Analog
                Constraint::Percentage(50), // Digital
            ]
            .as_ref(),
        )
        .split(area);

    render_digital_io_block(f, tpu, chunks[0]);
    render_analog_io_block(f, tpu, chunks[1]);
    // // For now, just display a placeholder
    // let widget = Paragraph::new("I/O Pin states will be displayed here")
    //     .block(Block::default().borders(Borders::ALL).title("I/O Pins"));
    //    f.render_widget(widget, area);
}

fn render_digital_io_block(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let constraints = DigitalPin::iter().map(|_| Constraint::Fill(1));

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for pin in DigitalPin::iter() {
        let state = tpu.digital_pins[pin as usize];
        let widget = Paragraph::new("")
            .style(Style::default().fg(Color::White).bg(if state {
                Color::Green
            } else {
                Color::Black
            }))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{pin:?}")),
            );
        f.render_widget(widget, chunks[pin as usize]);
    }
}

fn render_analog_io_block(f: &mut Frame, tpu: &tpu::TpuState, area: ratatui::layout::Rect) {
    let constraints = AnalogPin::iter().map(|_| Constraint::Fill(1));

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for pin in AnalogPin::iter() {
        let state = tpu.analog_pins[pin as usize];
        let widget = Paragraph::new(format!("{}", state))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{pin:?}")),
            );
        f.render_widget(widget, chunks[pin as usize]);
    }
}
