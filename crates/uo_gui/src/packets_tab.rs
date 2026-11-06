//! Static "Comunicação" reference tab: UO client 7.0.10.3 packet table and gump layout
//! commands, extracted by static analysis of client.exe (data embedded at compile time).

use eframe::egui;
use egui_extras::{Column, TableBuilder};

const PACKETS_MD: &str = include_str!("../data/client_packets.md");
const GUMP_CMDS_MD: &str = include_str!("../data/gump_commands.md");
const NOTES_MD: &str = include_str!("../data/protocol_notes.md");

#[derive(Clone, Debug)]
pub struct PacketRow {
    pub id: u32,
    pub id_text: String,
    pub size: String,
    pub handler: String,
    pub name: String,
    pub conf: String,
}

#[derive(Clone, Debug)]
pub struct GumpCmdRow {
    pub num: String,
    pub name: String,
    pub args: String,
    pub handler: String,
    pub notes: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    ServerToClient,
    ClientToServer,
    GumpCommands,
    Notes,
}

pub struct PacketsTab {
    s2c: Vec<PacketRow>,
    c2s: Vec<PacketRow>,
    gump_cmds: Vec<GumpCmdRow>,
    section: Section,
    search: String,
    only_var: bool,
    only_fixed: bool,
    min_conf: char, // 'A' = only A, 'B' = A+B, 'C' = all
    selected: Option<usize>,
}

fn cells(line: &str) -> Vec<String> {
    let t = line.trim().trim_start_matches('|').trim_end_matches('|');
    t.split('|').map(|s| s.trim().to_string()).collect()
}

fn parse_packets(md: &str) -> (Vec<PacketRow>, Vec<PacketRow>) {
    let (mut s2c, mut c2s) = (Vec::new(), Vec::new());
    let mut cur_s2c = true;
    for line in md.lines() {
        if line.starts_with("## ") {
            cur_s2c = line.contains("Servidor");
            continue;
        }
        if !line.starts_with("| 0x") {
            continue;
        }
        let c = cells(line);
        let id = u32::from_str_radix(c[0].trim_start_matches("0x"), 16).unwrap_or(0);
        let row = if cur_s2c && c.len() >= 5 {
            PacketRow { id, id_text: c[0].clone(), size: c[1].clone(), handler: c[2].clone(), name: c[3].clone(), conf: c[4].clone() }
        } else if !cur_s2c && c.len() >= 4 {
            PacketRow { id, id_text: c[0].clone(), size: c[1].clone(), handler: String::new(), name: c[2].clone(), conf: c[3].clone() }
        } else {
            continue;
        };
        if cur_s2c { s2c.push(row) } else { c2s.push(row) }
    }
    (s2c, c2s)
}

fn parse_gump_cmds(md: &str) -> Vec<GumpCmdRow> {
    md.lines()
        .filter(|l| l.starts_with("| ") && l.len() > 2 && l.as_bytes()[2].is_ascii_digit())
        .filter_map(|l| {
            let c = cells(l);
            (c.len() >= 5).then(|| GumpCmdRow {
                num: c[0].clone(),
                name: c[1].clone(),
                args: c[2].clone(),
                handler: c[3].clone(),
                notes: c[4].clone(),
            })
        })
        .collect()
}

impl Default for PacketsTab {
    fn default() -> Self {
        let (s2c, c2s) = parse_packets(PACKETS_MD);
        Self {
            s2c,
            c2s,
            gump_cmds: parse_gump_cmds(GUMP_CMDS_MD),
            section: Section::ServerToClient,
            search: String::new(),
            only_var: false,
            only_fixed: false,
            min_conf: 'C',
            selected: None,
        }
    }
}

impl PacketsTab {
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.s2c.len(), self.c2s.len(), self.gump_cmds.len())
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("packets_top").show_inside(ui, |ui| {
            ui.add_space(4.0);
            ui.heading(t!("protocol.heading"));
            ui.horizontal(|ui| {
                let n = (self.s2c.len(), self.c2s.len(), self.gump_cmds.len());
                for (sec, label) in [
                    (Section::ServerToClient, t!("protocol.sec.s2c", n = n.0)),
                    (Section::ClientToServer, t!("protocol.sec.c2s", n = n.1)),
                    (Section::GumpCommands, t!("protocol.sec.gump", n = n.2)),
                    (Section::Notes, t!("protocol.sec.notes")),
                ] {
                    if ui.selectable_label(self.section == sec, label).clicked() {
                        self.section = sec;
                        self.selected = None;
                    }
                }
            });
            if matches!(self.section, Section::ServerToClient | Section::ClientToServer | Section::GumpCommands) {
                ui.horizontal(|ui| {
                    ui.label(t!("common.search_icon"));
                    ui.text_edit_singleline(&mut self.search)
                        .on_hover_text(t!("protocol.search.tip"));
                    if self.section != Section::GumpCommands {
                        ui.checkbox(&mut self.only_var, t!("protocol.only_var"));
                        ui.checkbox(&mut self.only_fixed, t!("protocol.only_fixed"));
                        ui.label(t!("protocol.conf"));
                        egui::ComboBox::from_id_salt("pk_conf")
                            .selected_text(match self.min_conf {
                                'A' => t!("protocol.conf.a"),
                                'B' => t!("protocol.conf.ab"),
                                _ => t!("protocol.conf.all"),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.min_conf, 'A', t!("protocol.conf.a"));
                                ui.selectable_value(&mut self.min_conf, 'B', t!("protocol.conf.ab"));
                                ui.selectable_value(&mut self.min_conf, 'C', t!("protocol.conf.all"));
                            });
                    }
                });
            }
            ui.add_space(4.0);
        });

        match self.section {
            Section::Notes => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(t!("protocol.notes.summary"));
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new(NOTES_MD).monospace());
                    ui.separator();
                    ui.label(egui::RichText::new(GUMP_CMDS_MD.split("## gumppic").nth(1).map(|s| format!("## gumppic{s}")).unwrap_or_default()).monospace());
                });
            }
            Section::GumpCommands => self.show_gump_cmds(ui),
            sec => self.show_packets(ui, sec == Section::ServerToClient),
        }
    }

    fn conf_ok(&self, conf: &str) -> bool {
        let c = conf.chars().next().unwrap_or('C');
        match self.min_conf {
            'A' => c == 'A',
            'B' => c == 'A' || c == 'B',
            _ => true,
        }
    }

    fn show_packets(&mut self, ui: &mut egui::Ui, s2c: bool) {
        let q = self.search.trim().to_lowercase();
        let rows: Vec<PacketRow> = {
            let src = if s2c { &self.s2c } else { &self.c2s };
            src.iter()
                .filter(|r| {
                    let var = r.size.eq_ignore_ascii_case("var");
                    (!self.only_var || var)
                        && (!self.only_fixed || !var)
                        && self.conf_ok(&r.conf)
                        && (q.is_empty()
                            || r.id_text.to_lowercase().contains(&q)
                            || r.name.to_lowercase().contains(&q)
                            || r.handler.to_lowercase().contains(&q))
                })
                .cloned()
                .collect()
        };

        if let Some(i) = self.selected.filter(|&i| i < rows.len()) {
            let r = &rows[i];
            egui::TopBottomPanel::bottom("packets_detail").show_inside(ui, |ui| {
                ui.add_space(4.0);
                ui.label(egui::RichText::new(format!("{}  {}", r.id_text, r.name)).strong().size(16.0));
                ui.label(t!(
                    "protocol.detail",
                    dir = if s2c { t!("protocol.dir.s2c") } else { t!("protocol.dir.c2s") },
                    size = r.size,
                    handler = if r.handler.is_empty() { String::new() } else { t!("protocol.detail.handler", handler = r.handler) },
                    conf = r.conf
                ));
                ui.label(t!("protocol.detail.id", id = r.id, hex = format!("{:02X}", r.id)));
                ui.add_space(4.0);
            });
        }

        let mut clicked = None;
        TableBuilder::new(ui)
            .striped(true)
            .sense(egui::Sense::click())
            .column(Column::exact(60.0))
            .column(Column::exact(50.0))
            .column(Column::exact(90.0))
            .column(Column::remainder().at_least(200.0))
            .column(Column::exact(40.0))
            .header(22.0, |mut h| {
                for t in [t!("protocol.col.id"), t!("protocol.col.size"), t!("protocol.col.handler"), t!("protocol.col.name"), t!("protocol.col.conf")] {
                    h.col(|ui| {
                        ui.strong(t);
                    });
                }
            })
            .body(|body| {
                body.rows(20.0, rows.len(), |mut row| {
                    let i = row.index();
                    let r = &rows[i];
                    row.set_selected(self.selected == Some(i));
                    row.col(|ui| {
                        ui.monospace(&r.id_text);
                    });
                    row.col(|ui| {
                        ui.label(&r.size);
                    });
                    row.col(|ui| {
                        ui.monospace(&r.handler);
                    });
                    row.col(|ui| {
                        ui.label(&r.name);
                    });
                    row.col(|ui| {
                        ui.label(&r.conf);
                    });
                    if row.response().clicked() {
                        clicked = Some(i);
                    }
                });
            });
        if clicked.is_some() {
            self.selected = clicked;
        }
    }

    fn show_gump_cmds(&mut self, ui: &mut egui::Ui) {
        let q = self.search.trim().to_lowercase();
        let rows: Vec<&GumpCmdRow> = self
            .gump_cmds
            .iter()
            .filter(|r| q.is_empty() || r.name.to_lowercase().contains(&q) || r.notes.to_lowercase().contains(&q))
            .collect();
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::exact(30.0))
            .column(Column::exact(140.0))
            .column(Column::exact(190.0))
            .column(Column::exact(170.0))
            .column(Column::remainder().at_least(200.0))
            .header(22.0, |mut h| {
                for t in [t!("protocol.gcol.num"), t!("protocol.gcol.cmd"), t!("protocol.gcol.args"), t!("protocol.gcol.handler"), t!("protocol.gcol.notes")] {
                    h.col(|ui| {
                        ui.strong(t);
                    });
                }
            })
            .body(|body| {
                body.rows(20.0, rows.len(), |mut row| {
                    let r = rows[row.index()];
                    row.col(|ui| {
                        ui.label(&r.num);
                    });
                    row.col(|ui| {
                        ui.monospace(&r.name);
                    });
                    row.col(|ui| {
                        ui.label(&r.args);
                    });
                    row.col(|ui| {
                        ui.monospace(&r.handler);
                    });
                    row.col(|ui| {
                        ui.label(&r.notes);
                    });
                });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_tables_parse() {
        let t = PacketsTab::default();
        let (s, c, g) = t.counts();
        assert!(s > 100, "s2c rows: {s}");
        assert!(c > 50, "c2s rows: {c}");
        assert_eq!(g, 27);
        assert!(t.s2c.iter().any(|r| r.id == 0x1A && r.size == "var"));
        assert!(t.c2s.iter().any(|r| r.id == 0x02 && r.size == "7"));
    }
}
