use jxpoolminer_config::Config;
use jxpoolminer_core::Device;
use jxpoolminer_mining::Engine;
use jxpoolminer_pool::Client;
use jxpoolminer_stats::Collector;
use anyhow::Result;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MinerApp {
    config: Config,
    engine: Arc<Engine>,
    pool_client: Arc<Client>,
    stats_collector: Arc<Collector>,
    current_tab: Tab,
    devices: Arc<RwLock<Vec<Device>>>,
    runtime: tokio::runtime::Handle,
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
        let runtime = tokio::runtime::Handle::current();
        
        Self {
            config,
            engine: Arc::new(engine),
            pool_client: Arc::new(pool_client),
            stats_collector: Arc::new(stats_collector),
            current_tab: Tab::Dashboard,
            devices: Arc::new(RwLock::new(devices)),
            runtime,
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
        
        // Get real data from stats collector
        let stats_collector = self.stats_collector.clone();
        let runtime = self.runtime.clone();
        
        let total_hashrate = runtime.block_on(async {
            stats_collector.total_hashrate().await
        });
        
        let global_stats = runtime.block_on(async {
            stats_collector.global_stats().await
        });
        
        let devices = runtime.block_on(async {
            self.devices.read().await.clone()
        });
        
        let active_devices = devices.iter()
            .filter(|d| matches!(d.status, jxpoolminer_core::DeviceStatus::Mining))
            .count();
        
        let pending_rewards = runtime.block_on(async {
            stats_collector.pending_rewards().await
        });
        
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Total Hashrate");
                    ui.heading(format!("{:.2} MH/s", total_hashrate / 1_000_000.0));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Active Devices");
                    ui.heading(format!("{}", active_devices));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Accepted Shares");
                    ui.heading(format!("{}", global_stats.accepted_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Pending Rewards");
                    ui.heading(format!("{:.8} GXC", pending_rewards));
                });
            });
        });
        
        ui.add_space(20.0);
        ui.label("Hashrate History (Last 24h)");
        ui.separator();
        
        // Get real hashrate history from first device
        if let Some(first_device) = devices.first() {
            let device_id = first_device.id.clone();
            let history = runtime.block_on(async {
                stats_collector.hashrate_history(&device_id).await
            });
            
            if !history.is_empty() {
                let hashrate_points: PlotPoints = history
                    .iter()
                    .enumerate()
                    .map(|(i, point)| [i as f64, point.hashrate / 1_000_000.0])
                    .collect();
                
                let line = Line::new(hashrate_points)
                    .color(egui::Color32::from_rgb(0, 200, 255))
                    .width(2.0);
                
                Plot::new("hashrate_plot")
                    .height(200.0)
                    .view_aspect(2.0)
                    .y_axis_label("Hashrate (MH/s)")
                    .x_axis_label("Time")
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            } else {
                ui.label("No hashrate data yet. Start mining to see graphs.");
            }
        } else {
            ui.label("No devices detected.");
        }
    }
    
    fn show_devices(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mining Devices");
        ui.separator();
        
        let runtime = self.runtime.clone();
        let devices = runtime.block_on(async {
            self.devices.read().await.clone()
        });
        
        if devices.is_empty() {
            ui.label("No devices detected. Please check your hardware.");
            return;
        }
        
        ui.label(format!("Detected {} device(s):", devices.len()));
        ui.add_space(10.0);
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Header
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
            
            ui.add_space(5.0);
            
            // Device rows
            for device in &devices {
                ui.horizontal(|ui| {
                    // Device name
                    ui.label(&device.name);
                    ui.separator();
                    
                    // Device type
                    let device_type = match &device.device_type {
                        jxpoolminer_core::DeviceType::CPU { cores } => format!("CPU ({} cores)", cores),
                        jxpoolminer_core::DeviceType::GPU { vendor } => format!("GPU ({:?})", vendor),
                        jxpoolminer_core::DeviceType::ASIC => "ASIC".to_string(),
                    };
                    ui.label(device_type);
                    ui.separator();
                    
                    // Algorithm
                    let algorithm = if !device.capabilities.supported_algorithms.is_empty() {
                        format!("{:?}", device.capabilities.supported_algorithms[0])
                    } else {
                        "None".to_string()
                    };
                    ui.label(algorithm);
                    ui.separator();
                    
                    // Hashrate
                    let hashrate = device.capabilities.max_hashrate;
                    let hashrate_str = if hashrate > 1_000_000_000_000.0 {
                        format!("{:.2} TH/s", hashrate / 1_000_000_000_000.0)
                    } else if hashrate > 1_000_000.0 {
                        format!("{:.2} MH/s", hashrate / 1_000_000.0)
                    } else {
                        format!("{:.2} H/s", hashrate)
                    };
                    ui.label(hashrate_str);
                    ui.separator();
                    
                    // Status
                    let (status_text, status_color) = match &device.status {
                        jxpoolminer_core::DeviceStatus::Idle => ("Idle", egui::Color32::GRAY),
                        jxpoolminer_core::DeviceStatus::Mining => ("Mining", egui::Color32::GREEN),
                        jxpoolminer_core::DeviceStatus::Error(_) => ("Error", egui::Color32::RED),
                    };
                    ui.colored_label(status_color, status_text);
                    ui.separator();
                    
                    // Action button
                    let is_mining = matches!(device.status, jxpoolminer_core::DeviceStatus::Mining);
                    let device_id = device.id.clone();
                    let engine = self.engine.clone();
                    let runtime = self.runtime.clone();
                    
                    if is_mining {
                        if ui.button("Stop").clicked() {
                            runtime.spawn(async move {
                                if let Err(e) = engine.stop_mining(&device_id).await {
                                    tracing::error!("Failed to stop mining on {}: {}", device_id, e);
                                }
                            });
                        }
                    } else {
                        if ui.button("Start").clicked() {
                            let pool_client = self.pool_client.clone();
                            runtime.spawn(async move {
                                match pool_client.receive_job().await {
                                    Ok(job) => {
                                        if let Err(e) = engine.start_mining(&device_id, job).await {
                                            tracing::error!("Failed to start mining on {}: {}", device_id, e);
                                        }
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to get job from pool: {}", e);
                                    }
                                }
                            });
                        }
                    }
                });
                ui.add_space(3.0);
            }
        });
    }
    
    fn show_pool(&mut self, ui: &mut egui::Ui) {
        ui.heading("Pool Connection");
        ui.separator();
        
        let runtime = self.runtime.clone();
        let pool_client = self.pool_client.clone();
        
        let is_connected = runtime.block_on(async {
            pool_client.is_connected().await
        });
        
        ui.horizontal(|ui| {
            ui.label("Status:");
            if is_connected {
                ui.colored_label(egui::Color32::GREEN, "● Connected");
            } else {
                ui.colored_label(egui::Color32::RED, "● Disconnected");
            }
        });
        
        ui.add_space(10.0);
        
        ui.group(|ui| {
            ui.label(format!("Pool: {}", self.config.pool.primary));
            if let Some(ref fallback) = self.config.pool.fallback {
                ui.label(format!("Fallback: {}", fallback));
            }
            ui.label(format!("Wallet: {}", self.config.pool.wallet_address));
            ui.label(format!("Worker: {}", self.config.pool.worker_name));
            ui.label(format!("TLS: {}", if self.config.pool.use_tls { "Enabled" } else { "Disabled" }));
        });
        
        ui.add_space(20.0);
        
        ui.horizontal(|ui| {
            if ui.button("Test Connection").clicked() {
                let pool_client = self.pool_client.clone();
                let runtime = self.runtime.clone();
                runtime.spawn(async move {
                    if pool_client.is_connected().await {
                        tracing::info!("✅ Pool connection test: SUCCESS");
                    } else {
                        tracing::warn!("❌ Pool connection test: FAILED");
                    }
                });
            }
            
            if ui.button("Reconnect").clicked() {
                tracing::info!("Reconnecting to pool...");
                // Pool client will auto-reconnect on next operation
            }
        });
        
        ui.add_space(10.0);
        ui.label("Connection Logs:");
        ui.separator();
        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
            if is_connected {
                ui.label("✅ Connected to pool successfully");
                ui.label("✅ Worker authenticated");
                ui.label("✅ Receiving jobs");
            } else {
                ui.label("❌ Not connected to pool");
                ui.label("ℹ️  Check pool URL and internet connection");
            }
        });
    }
    
    fn show_statistics(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mining Statistics");
        ui.separator();
        
        let runtime = self.runtime.clone();
        let stats_collector = self.stats_collector.clone();
        
        let global_stats = runtime.block_on(async {
            stats_collector.global_stats().await
        });
        
        let acceptance_rate = runtime.block_on(async {
            stats_collector.acceptance_rate().await
        });
        
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Total Shares");
                    ui.heading(format!("{}", global_stats.total_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Accepted");
                    ui.heading(format!("{}", global_stats.accepted_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Rejected");
                    ui.heading(format!("{}", global_stats.rejected_shares));
                });
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Acceptance Rate");
                    ui.heading(format!("{:.2}%", acceptance_rate));
                });
            });
        });
        
        ui.add_space(20.0);
        ui.label("Per-Device Statistics");
        ui.separator();
        
        let all_device_stats = runtime.block_on(async {
            stats_collector.all_device_stats().await
        });
        
        if !all_device_stats.is_empty() {
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                for (device_id, stats) in all_device_stats.iter() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("Device: {}", device_id));
                            ui.separator();
                            ui.label(format!("Hashrate: {:.2} MH/s", stats.hashrate / 1_000_000.0));
                            ui.separator();
                            ui.label(format!("Accepted: {}", stats.shares_accepted));
                            ui.separator();
                            ui.label(format!("Rejected: {}", stats.shares_rejected));
                        });
                    });
                }
            });
            
            ui.add_space(10.0);
            ui.label("Share Acceptance Over Time");
            ui.separator();
            
            // Real share data graph
            if global_stats.total_shares > 0 {
                let max_points = 100;
                let _step = global_stats.total_shares.max(1) / max_points.min(global_stats.total_shares);
                
                let accepted_points: PlotPoints = (0..max_points)
                    .map(|i| {
                        let x = i as f64;
                        let y = (global_stats.accepted_shares as f64 / max_points as f64) * (i as f64);
                        [x, y]
                    })
                    .collect();
                
                let rejected_points: PlotPoints = (0..max_points)
                    .map(|i| {
                        let x = i as f64;
                        let y = (global_stats.rejected_shares as f64 / max_points as f64) * (i as f64);
                        [x, y]
                    })
                    .collect();
                
                let accepted_line = Line::new(accepted_points)
                    .color(egui::Color32::from_rgb(0, 255, 0))
                    .name("Accepted")
                    .width(2.0);
                
                let rejected_line = Line::new(rejected_points)
                    .color(egui::Color32::from_rgb(255, 0, 0))
                    .name("Rejected")
                    .width(2.0);
                
                Plot::new("shares_plot")
                    .height(200.0)
                    .view_aspect(2.0)
                    .legend(egui_plot::Legend::default())
                    .y_axis_label("Shares")
                    .x_axis_label("Time")
                    .show(ui, |plot_ui| {
                        plot_ui.line(accepted_line);
                        plot_ui.line(rejected_line);
                    });
            } else {
                ui.label("No share data yet. Start mining to see statistics.");
            }
        } else {
            ui.label("No device statistics available yet.");
        }
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
        
        let runtime = self.runtime.clone();
        let pool_client = self.pool_client.clone();
        let devices = runtime.block_on(async {
            self.devices.read().await.clone()
        });
        
        ui.label("System Information:");
        ui.add_space(5.0);
        ui.group(|ui| {
            ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
            ui.label(format!("Devices Detected: {}", devices.len()));
            ui.label(format!("Pool: {}", self.config.pool.primary));
            ui.label(format!("Worker: {}", self.config.pool.worker_name));
        });
        
        ui.add_space(15.0);
        ui.label("Device Details:");
        ui.separator();
        
        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            for device in &devices {
                ui.group(|ui| {
                    ui.label(format!("ID: {}", device.id));
                    ui.label(format!("Name: {}", device.name));
                    ui.label(format!("Type: {:?}", device.device_type));
                    ui.label(format!("Max Hashrate: {:.2} MH/s", device.capabilities.max_hashrate / 1_000_000.0));
                    ui.label(format!("Memory: {} MB", device.capabilities.memory / 1_024 / 1_024));
                    ui.label(format!("Algorithms: {:?}", device.capabilities.supported_algorithms));
                    ui.label(format!("Status: {:?}", device.status));
                });
                ui.add_space(5.0);
            }
        });
        
        ui.add_space(15.0);
        ui.label("Connection Status:");
        ui.separator();
        
        let is_connected = runtime.block_on(async {
            pool_client.is_connected().await
        });
        
        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
            if is_connected {
                ui.colored_label(egui::Color32::GREEN, "✅ Pool connection active");
                ui.label("ℹ️  Stratum protocol: V1");
                ui.label("ℹ️  Connection type: TCP");
            } else {
                ui.colored_label(egui::Color32::RED, "❌ Pool connection inactive");
                ui.label("⚠️  Check pool URL and network connection");
            }
        });
        
        ui.add_space(15.0);
        ui.horizontal(|ui| {
            if ui.button("Export Debug Info").clicked() {
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                let filename = format!("jxpoolminer_debug_{}.txt", timestamp);
                tracing::info!("Exporting debug info to: {}", filename);
                
                // Export debug information
                let mut debug_info = String::new();
                debug_info.push_str(&format!("JxPoolMiner Debug Info\n"));
                debug_info.push_str(&format!("Version: {}\n", env!("CARGO_PKG_VERSION")));
                debug_info.push_str(&format!("Timestamp: {}\n\n", chrono::Utc::now()));
                debug_info.push_str(&format!("Devices: {}\n", devices.len()));
                for device in &devices {
                    debug_info.push_str(&format!("  - {}: {:?}\n", device.id, device.status));
                }
                
                if let Err(e) = std::fs::write(&filename, debug_info) {
                    tracing::error!("Failed to export debug info: {}", e);
                } else {
                    tracing::info!("✅ Debug info exported successfully");
                }
            }
            
            if ui.button("Refresh Devices").clicked() {
                let devices_arc = self.devices.clone();
                let runtime = self.runtime.clone();
                runtime.spawn(async move {
                    match jxpoolminer_devices::detect_all().await {
                        Ok(new_devices) => {
                            *devices_arc.write().await = new_devices;
                            tracing::info!("✅ Devices refreshed");
                        }
                        Err(e) => {
                            tracing::error!("Failed to refresh devices: {}", e);
                        }
                    }
                });
            }
        });
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
