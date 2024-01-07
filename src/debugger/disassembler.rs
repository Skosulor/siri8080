use std::io;
use std::io::Write;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Row, Block, Borders, List , Text, Table};
use tui::style::{Style, Color};
use tui::Terminal;
use crate::i8080::registers::Registers;
use crate::i8080::flags::StatusFlags;

// const MEMORY_ROWS: usize = 44;
// pub const N_MEM_BYTES: usize = 44*16;

pub struct Term<'a>
{
    mem:   Vec<Vec<&'a str>>,
    inst:  Vec<String>,
    flags: Vec<Vec<&'a str>>,
    regs:  Vec<Vec<String>>,
    pc:    Vec<Vec<String>>,
}

impl<'a> Term<'a>
{
    pub fn default() -> Term<'a>
    {
        let t = Term
        {
            mem: vec![ vec!["00"; 17]; 44],
            inst: Vec::new(),
            regs: vec![
                vec!["Accumulator".to_string(), "B".to_string(),"C".to_string()],
                vec!["0".to_string(), "0".to_string(), "0".to_string()],
                vec!["".to_string()],
                vec!["D".to_string(),"E".to_string(), "H".to_string(), "L".to_string()],
                vec!["0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],],
            flags: vec![
                vec!["Sign", "Zero","Carry"],
                vec!["0", "0", "0"],
                vec!["Auxiliary","Parity"],
                vec!["0", "0"],],
            pc: vec![
                vec!["PC".to_string(), "D. Addr".to_string(), "Immediate".to_string()],
                vec!["0".to_string(), "0".to_string(), "0".to_string()],
                vec!["0x0".to_string(), "0x0".to_string(),"0x0".to_string()],
                vec!["".to_string()],
                vec!["SP".to_string()],
                vec!["0x0".to_string()],
                vec!["0".to_string()],
            ],
        };
        t
    }

    pub fn set_stack_pointer(&mut self, stack_pointer: u16)
    {
        self.pc[5][0] = stack_pointer.to_string();
        self.pc[6][0] = format!("0x{:04X}", stack_pointer);
    }

    pub fn set_direct_address(&mut self, address: u16)
    {
        self.pc[1][1] = address.to_string();
        self.pc[2][1] = format!("0x{:04X}", address);
    }

    pub fn set_immediate(&mut self, immediate: u8)
    {
        self.pc[1][2] = immediate.to_string();
        self.pc[2][2] = format!("0x{:02X}", immediate);
    }

    pub fn set_pc(&mut self, pc: u16)
    {
        self.pc[1][0] = pc.to_string();
        self.pc[2][0] = format!("0x{:04X}", pc);
    }

    pub fn set_regs(&mut self, reg: &Registers)
    {
        self.regs[1][0] = (reg.accumulator).to_string();
        self.regs[1][1] = (reg.b).to_string();
        self.regs[1][2] = (reg.c).to_string();
        self.regs[4][0] = (reg.d).to_string();
        self.regs[4][1] = (reg.e).to_string();
        self.regs[4][2] = (reg.h).to_string();
        self.regs[4][3] = (reg.l).to_string();

    }

    pub fn set_flags(&mut self, f: &StatusFlags)
    {
        self.flags[1][0] = match f.sign_flag      { true => "1", _ => "0" };
        self.flags[1][1] = match f.zero_flag      { true => "1", _ => "0" };
        self.flags[1][2] = match f.carry_flag     { true => "1", _ => "0" };
        self.flags[3][0] = match f.auxiliary_flag { true => "1", _ => "0" };
        self.flags[3][1] = match f.parity_flag    { true => "1", _ => "0" };
    }

    pub fn update_instructions(&mut self, instructions: Vec<String>)
    {
        self.inst.clear();
        for x in instructions.iter()
        {
            self.inst.push(x.clone());
        }
    }

    pub fn update_dissambler(&self) 
    {
        // Initiate
        let stdout       = io::stdout().into_raw_mode().expect("IO error");
        let backend      = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Failed to create new Terminal (Tui)");

        let instructions_text = self.inst.iter().map(|i| Text::raw(i));
        let instructions  = List::new(instructions_text).style(Style::default());

        // memory_values
        let row_style = Style::default().fg(Color::White);

        // Mem memory_values
        let mem_rows = self.mem
            .iter()
            .map(|i| Row::StyledData(i.iter(), row_style));

        let pc_rows = 
            self.pc
            .iter()
            .map(|i| Row::StyledData(i.iter(), row_style));

        let t = Table::new(["Address", "0x0","0x1", "0x2", "0x3", "0x4", "0x5", "0x6", "0x7", "0x8",
                           "0x9", "0xA", "0xB", "0xC", "0xD", "0xE", "0xF"].iter(),mem_rows);
        let memory_values = t
            .block(Block::default().title(""))
            .header_style(Style::default().fg(Color::Yellow))
            .widths(&[Constraint::Length(10), Constraint::Length(4),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3),Constraint::Length(3)])
            .style(Style::default().fg(Color::White))
            .column_spacing(1);

        // Flags memory_values
        let header = ["", ""];
        let flag_rows = self.flags
            .iter()
            .map(|i| Row::StyledData(i.iter(), row_style));

        let flag_values = Table::new(header.iter(), flag_rows)
            .block(Block::default().borders(Borders::NONE).title(""))
            .highlight_symbol(">> ")
            .widths(&[
                    Constraint::Length(15),
                    Constraint::Length(15),
                    Constraint::Length(15),
            ]);


        let reg_rows = self.regs
            .iter()
            .map(|i| Row::StyledData(i.iter(), row_style));

        let register_values = Table::new(header.iter(), reg_rows)
            .block(Block::default().borders(Borders::NONE).title(""))
            .highlight_symbol(">> ")
            .widths(&[
                    Constraint::Length(15),
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
            ]);

        let pc_values = Table::new(header.iter(), pc_rows)
            .block(Block::default().borders(Borders::NONE).title(""))
            .highlight_symbol(">> ")
            .widths(&[
                    Constraint::Length(15),
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length(10),
            ]);


        let box_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25)].as_ref());

        let box_layout_inst = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(30)].as_ref());


        let mem_pc_border       = Block::default().title("Memory").borders(Borders::ALL);
        let pc_border           = Block::default().title("PC").borders(Borders::ALL);
        let shell_border        = Block::default().title("Shell").borders(Borders::ALL);
        let registers_border    = Block::default().title("Registers").borders(Borders::ALL);
        let flags_border        = Block::default().title("Flags").borders(Borders::ALL);
        let instructions_border = Block::default().title("Instructions").borders(Borders::ALL);

        let mut shell_widget_position = Default::default();

        terminal.draw(|mut f|{

            let rect = f.size();
            let y = rect.top();
            let x = rect.left();
            let width = rect.right();
            let height = rect.bottom();

            let mem_rect = Rect::new(y, x, width/2, height);
            let inst_rect = Rect::new(y+width/2+width/4, x, width/4, height);
            let rect = Rect::new(y+width/2, x, width/4, height);
            let rect_in = Rect::new(y+width/2, x, width/4, height);

            let box_multi = box_layout.clone().split(rect);
            let box_multi_in = box_layout.margin(1).horizontal_margin(3).clone().split(rect_in);

            // Right box
            let box_inst = box_layout_inst.clone().split(inst_rect);
            let box_inst_in = box_layout_inst.horizontal_margin(2).vertical_margin(1).split(inst_rect);


            let box_layout_memory = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(30)].as_ref())
                .split(mem_rect);

            let layout_memory = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(3)
                .horizontal_margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(mem_rect);

            // left: memory
            f.render_widget(mem_pc_border, box_layout_memory[0]);
            f.render_widget(memory_values, layout_memory[0]);

            f.render_widget(pc_border, box_multi[0]);
            f.render_widget(pc_values, box_multi_in[0]);

            f.render_widget(registers_border, box_multi[1]);
            f.render_widget(register_values, box_multi_in[1]);

            f.render_widget(flags_border, box_multi[2]);
            f.render_widget(flag_values, box_multi_in[2]);


            f.render_widget(shell_border, box_multi[3]);
            f.render_widget(instructions_border, box_inst[0]);
            f.render_widget(instructions, box_inst_in[0]);

            shell_widget_position = box_multi[3];

        }).expect("Failed to draw!");

        let cursor_x = shell_widget_position.x + 3;
        let cursor_y = shell_widget_position.y + 3;
        print!("{}", termion::cursor::Goto(cursor_x, cursor_y));
        terminal.backend_mut().flush().expect("Failed to flush backend");
        // backend.flush().expect("Failed to flush backend");
    }
}



