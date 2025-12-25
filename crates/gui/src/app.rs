use jxpoolminer_config::Config;
use jxpoolminer_core::Device;
use jxpoolminer_mining::Engine;
use jxpoolminer_pool::Client;
use jxpoolminer_stats::Collector;
use anyhow::Result;
use eframe::egui;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MinerApp {
    config: Config,
    engine: Arc<Engine>,
    pool_client: Arc<Client>,
    stats_collector: Arc<Collector>,
    current_tab: Tab,
    devices: Arc<RwLock<Vec<Device>>>,
    hashrate_history: Vec<f64>,
    total_shares: u64,
    accepted_shares: u64,
    rejected_shares: u64,
}

#[derive(PartialEq)]
enum Tab {
    Dashboard,
    Devices,
    Pool,
    Statistics,
    Settings,
    Advanced,
}

impl MinerApp {
    pub fn new(
        config: Config,
        devices: Vec<Device>,
        engine: Engine,
        pool_client: Client,
        stats_collector: Collector,
    ) -> Self {
        Self {
            config,
            engine: Arc::new(engine),
            pool_client: Arc::new(pool_client),
            stats_collector: Arc::new(stats_collector),
            current_tab: Tab::Dashboard,
            devices: Arc::new(RwLock::new(devices)),
            hashrate_history: Vec::new(),
            total_shares: 0,
            accepted_shares: 0,
            rejected_shares: 0,
        }
    }
}

impl eframe::App for MinerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🔷 JxPoolMiner");
                ui.separator();
                
                if ui.selectable_label(self.current_tab == Tab::Dashboard, "📊 Dashboard").clicked() {
                    self.current_tab = Tab::Dashboard;
                }
                if ui.selectable_label(self.current_tab == Tab::Devices, "🖥️ Devices").clicked() {
                    self.current_tab = Tab::Devices;
                }
                if ui.selectable_label(self.current_tab == Tab::Pool, "🌐 Pool").clicked() {
                    self.current_tab = Tab::Pool;
                }
                if ui.selectable_label(self.current_tab == Tab::Statistics, "📈 Statistics").clicked() {
                    self.current_tab = Tab::Statistics;
                }
                if ui.selectable_label(self.current_tab == Tab::Settings, "⚙️ Settings").clicked() {
                    self.current_tab = Tab::Settings;
                }
                if ui.selectable_label(self.current_tab == Tab::Advanced, "🔧 Advanced").clicked() {
                    self.current_tab = Tab::Advanced;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Dashboard => self.show_dashboard(ui),
                Tab::Devices => self.show_devices(ui),
                Tab::Pool => self.show_pool(ui),
                Tab::Statistics => self.show_statistics(ui),
                Tab::Settings => self.show_settings(ui),
                Tab::Advanced => self.show_advanced(ui),
            }
        });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

impl MinerApp {
    fn show_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dashboard");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Total Hashrate");
                    ui.heading("125.5 MH/s");
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Active Devices");
                    ui.heading("3");
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Accepted Shares");
                    ui.heading(format!("{}", self.accepted_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Pending Rewards");
                    ui.heading("0.0025 BTC");
                });
            });
        });
        
        ui.add_space(20.0);
        ui.label("Hashrate History (Last 24h)");
        ui.separator();
        ui.label("[Chart would be displayed here]");
    }
    
    fn show_devices(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mining Devices");
        ui.separator();
        
        ui.label("Detected devices:");
        ui.add_space(10.0);
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Device");
                    ui.separator();
                    ui.label("Type");
                    ui.separator();
                    ui.label("Algorithm");
                    ui.separator();
                    ui.label("Hashrate");
                    ui.separator();
                    ui.label("Status");
                    ui.separator();
                    ui.label("Action");
                });
            });
            
            ui.label("CPU-0: Intel Core i7 (GXHash) - 15 MH/s - Idle [Start]");
            ui.label("GPU-0: NVIDIA RTX 3090 (Ethash) - 120 MH/s - Mining [Stop]");
            ui.label("ASIC-0: Antminer S19 (SHA-256) - 110 TH/s - Idle [Start]");
        });
    }
    
    fn show_pool(&mut self, ui: &mut egui::Ui) {
        ui.heading("Pool Connection");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.colored_label(egui::Color32::GREEN, "● Connected");
        });
        
        ui.add_space(10.0);
        
        ui.group(|ui| {
            ui.label(format!("Pool: {}", self.config.pool.primary));
            ui.label(format!("Worker: {}", self.config.pool.worker_name));
            ui.label("Latency: 45ms");
            ui.label("Difficulty: 16384");
        });
        
        ui.add_space(20.0);
        
        if ui.button("Test Connection").clicked() {
            // Test connection
        }
        
        if ui.button("Reconnect").clicked() {
            // Reconnect
        }
    }
    
    fn show_statistics(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mining Statistics");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Total Shares");
                    ui.heading(format!("{}", self.total_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Accepted");
                    ui.heading(format!("{}", self.accepted_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Rejected");
                    ui.heading(format!("{}", self.rejected_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Acceptance Rate");
                    let rate = if self.total_shares > 0 {
                        (self.accepted_shares as f64 / self.total_shares as f64) * 100.0
                    } else {
                        0.0
                    };
                    ui.heading(format!("{:.2}%", rate));
                });
            });
        });
        
        ui.add_space(20.0);
        ui.label("Per-Device Statistics");
        ui.separator();
        ui.label("[Device contribution charts would be displayed here]");
    }
    
    fn show_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();
        
        ui.label("Mining Configuration");
        ui.checkbox(&mut self.config.mining.auto_detect_devices, "Auto-detect devices");
        ui.checkbox(&mut self.config.mining.auto_assign_algorithms, "Auto-assign algorithms");
        
        ui.add_space(10.0);
        ui.label("Appearance");
        ui.horizontal(|ui| {
            ui.label("Theme:");
            ui.radio_value(&mut self.config.app.theme, "dark".to_string(), "Dark");
            ui.radio_value(&mut self.config.app.theme, "light".to_string(), "Light");
        });
        
        ui.add_space(20.0);
        if ui.button("Save Settings").clicked() {
            // Save settings
        }
    }
    
    fn show_advanced(&mut self, ui: &mut egui::Ui) {
        ui.heading("Advanced / Debug");
        ui.separator();
        
        ui.label("Recent Pool Messages:");
        ui.add_space(10.0);
        
        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            ui.label("[2024-12-25 06:30:15] → mining.subscribe");
            ui.label("[2024-12-25 06:30:15] ← mining.subscribe OK");
            ui.label("[2024-12-25 06:30:16] → mining.authorize");
            ui.label("[2024-12-25 06:30:16] ← mining.authorize OK");
            ui.label("[2024-12-25 06:30:17] ← mining.notify (new job)");
        });
        
        ui.add_space(20.0);
        if ui.button("Export Logs").clicked() {
            // Export logs
        }
    }
}

pub async fn run(
    config: Config,
    devices: Vec<Device>,
    mining_engine: Engine,
    pool_client: Client,
    stats_collector: Collector,
) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    let app = MinerApp::new(config, devices, mining_engine, pool_client, stats_collector);
    
    eframe::run_native(
        "JxPoolMiner",
        options,
        Box::new(|_cc| Box::new(app)),
    ).map_err(|e| anyhow::anyhow!("GUI error: {}", e))?;
    
    Ok(())
}
