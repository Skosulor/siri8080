use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Row, Block, Borders, List , Text, Table };
use tui::style::{Style, Color};
use tui::{Terminal};
use crate::i8080::registers::Registers;
use crate::i8080::flags::StatusFlags;

// const MEMORY_ROWS: usize = 44;
// pub const N_MEM_BYTES: usize = 44*16;

pub struct Term<'a>{
    mem: Vec<Vec<&'a str>>,
    inst: Vec<String>,
    flags: Vec<Vec<&'a str>>,
    regs: Vec<Vec<String>>,
}

impl<'a> Term<'a>{

    pub fn default() -> Term<'a>{
        let t = Term{
            mem: vec![ vec!["00"; 17]; 44],
            inst: Vec::new(),
            regs: vec![
                vec!["Accumulator".to_string(), "B".to_string(),"C".to_string()],
                vec!["0".to_string(), "0".to_string(), "0".to_string()],
                vec!["".to_string()],
                vec!["D".to_string(),"E".to_string(), "H".to_string(), "L".to_string()],
                vec!["0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
            ],
            flags: vec![
                vec!["Sign", "Zero","Carry"],
                vec!["0", "0", "0"],
                vec!["Auxiliary","Parity"],
                vec!["0", "0"],
            ],
        };
        t
    }

    // pub fn set_mem(&mut self){}

    pub fn set_regs(&mut self, reg: &Registers){
        self.regs[1][0] = (reg.accumulator).to_string();
        self.regs[1][1] = (reg.b).to_string();
        self.regs[1][2] = (reg.c).to_string();
        self.regs[4][0] = (reg.d).to_string();
        self.regs[4][1] = (reg.e).to_string();
        self.regs[4][2] = (reg.h).to_string();
        self.regs[4][3] = (reg.l).to_string();

    }

    pub fn set_flags(&mut self, f: &StatusFlags){

        self.flags[1][0] = match f.sign_flag      { true => "1", _ => "0" };
        self.flags[1][1] = match f.zero_flag      { true => "1", _ => "0" };
        self.flags[1][2] = match f.carry_flag     { true => "1", _ => "0" };
        self.flags[3][0] = match f.auxiliary_flag { true => "1", _ => "0" };
        self.flags[3][1] = match f.parity_flag    { true => "1", _ => "0" };

}
    pub fn update_instructions(&mut self, instructions: Vec<String>){
        self.inst.clear();
        for x in instructions.iter(){
           self.inst.push(x.clone());
       }
    }

    pub fn test_tui(&self) {
        // Remove rust compilation warning

        // Initiate
        let stdout       = io::stdout().into_raw_mode().expect("IO error");
        let backend      = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Failed to create new Terminal (Tui)");

        // List widget
        let i     = self.inst.clone();
        let items = i.iter().map(|i| Text::raw(i));
        let list  = List::new(items);
        let list  = list.style(Style::default());

        // Table
        let row_style = Style::default().fg(Color::White);

        // Mem Table
        let mem_rows = self.mem
                .iter()
                .map(|i| Row::StyledData(i.iter(), row_style));

        let t = Table::new(["Address", "0x0","0x1", "0x2", "0x3", "0x4", "0x5", "0x6", "0x7", "0x8",
                            "0x9", "0xA", "0xB", "0xC", "0xD", "0xE", "0xF"].iter(),mem_rows);
        let table = t
            .block(Block::default().title(""))
            .header_style(Style::default().fg(Color::Yellow))
            .widths(&[Constraint::Length(10), Constraint::Length(4),
                      Constraint::Length(4), Constraint::Length(4), Constraint::Length(4),
                      Constraint::Length(4), Constraint::Length(4), Constraint::Length(4),
                      Constraint::Length(4), Constraint::Length(4), Constraint::Length(4),
                      Constraint::Length(4), Constraint::Length(4), Constraint::Length(4),
                      Constraint::Length(4), Constraint::Length(4),Constraint::Length(4)])
            .style(Style::default().fg(Color::White))
            .column_spacing(1);

        // Flags Table
        let header = ["", ""];
        let flag_rows = self.flags
                .iter()
                .map(|i| Row::StyledData(i.iter(), row_style));

        let t = Table::new(header.iter(), flag_rows)
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

        let reg_t = Table::new(header.iter(), reg_rows)
            .block(Block::default().borders(Borders::NONE).title(""))
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Length(15),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
            ]);



        terminal.draw(|mut f| {
            let rect = f.size();
            let y = rect.top();
            let x = rect.left();
            let width = rect.right();
            let height = rect.bottom();

            let mem_rect = Rect::new(y, x, width/2, height);
            let inst_rect = Rect::new(y+width/2+width/4, x, width/4, height);
            let rect = Rect::new(y+width/2, x, width/4, height);
            let rect_in = Rect::new(y+width/2, x, width/4, height);


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


            // For middle boxes
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

            let test5 = list.clone();

            let mem_block = Block::default().title("Memory").borders(Borders::ALL);
            let block = Block::default().title("Unkown").borders(Borders::ALL);
            let reg = Block::default().title("Registers").borders(Borders::ALL);
            let flags = Block::default().title("Flags").borders(Borders::ALL);
            let ins = Block::default().title("Instructions").borders(Borders::ALL);

            // left: memory
            f.render_widget(mem_block, box_layout_memory[0]);
            f.render_widget(table, layout_memory[0]);

            f.render_widget(block, box_multi[0]);

            f.render_widget(reg, box_multi[1]);
            f.render_widget(reg_t, box_multi_in[1]);

            f.render_widget(flags, box_multi[2]);
            f.render_widget(t, box_multi_in[2]);

            f.render_widget(block, box_multi[3]);
           
             //Right: instructions
            f.render_widget(ins, box_inst[0]);
            f.render_widget(test5, box_inst_in[0]);

        }).expect("Failed to draw!");
    }

}



