// use gtk::{prelude::*, Application, ApplicationWindow, Box, Button, Label, Orientation};
// use std::process::{Command, Stdio};
use sysinfo::{ComponentExt, CpuExt, NetworkExt, System, SystemExt};

// Searching for CPU stuff
// fn lscpu_search(search: String, length: usize, ignore: bool) -> String {
//     let unfiltered = Command::new("lscpu")
//         .stdout(Stdio::piped())
//         .spawn()
//         .unwrap();

//     if ignore {
//         let filtered = Command::new("rg")
//             .arg("-i")
//             .arg(search)
//             .stdin(Stdio::from(unfiltered.stdout.unwrap()))
//             .stdout(Stdio::piped())
//             .spawn()
//             .unwrap();

//         let output = filtered.wait_with_output().unwrap();
//         String::from(
//             std::str::from_utf8(&output.stdout[length..output.stdout.len()])
//                 .unwrap()
//                 .trim(),
//         )
//     } else {
//         let filtered = Command::new("rg")
//             .arg(search)
//             .stdin(Stdio::from(unfiltered.stdout.unwrap()))
//             .stdout(Stdio::piped())
//             .spawn()
//             .unwrap();

//         let output = filtered.wait_with_output().unwrap();
//         String::from(
//             std::str::from_utf8(&output.stdout[length..output.stdout.len()])
//                 .unwrap()
//                 .trim(),
//         )
//     }
// }

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    loop {
        // First we update all information of our `System` struct.
        sys.refresh_all();

        // Network interfaces name, data received and data transmitted:
        println!("=> networks:");
        for (interface_name, data) in sys.networks() {
            println!(
                "{}: {}/{} B",
                interface_name,
                data.received(),
                data.transmitted()
            );
        }

        println!("=> system:");
        // RAM and swap information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

        println!();

        println!("=> CPU:");
        // Number of CPUs:
        println!("NB CPUs: {}", sys.cpus().len());

        // CPU frequency
        for cpu in sys.cpus() {
            println!("{}: {}%", cpu.name(), cpu.cpu_usage());
        }

        println!();
        // CPU Temperature
        for component in sys.components() {
            if component.label() == "zenpower Tdie" || component.label() == "k10temp Tdie" {
                println!("CPU temp: {}°C", component.temperature());
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // GTK Stuff
    //let app = Application::builder()
    //     .application_id("sysmontask-rs")
    //     .build();
    // app.connect_activate(build_ui);
    // app.run();
}

// Build the GTK app
// fn build_ui(app: &Application) {
//     let cpu_model = format!(
//         "CPU Model: {}",
//         lscpu_search(String::from("Model name"), 11, false)
//     );
//     let cpu_model = &cpu_model[0..cpu_model.len()];
//     let cpu_model_label = Label::builder()
//         .label(cpu_model)
//         .margin_top(3)
//         .margin_bottom(3)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     let l1_cache = format!("L1 Cache: {}", lscpu_search(String::from("L1d"), 10, false));
//     let l1_cache = &l1_cache[0..l1_cache.len()];
//     let l1_cache_label = Label::builder()
//         .label(l1_cache)
//         .margin_top(3)
//         .margin_bottom(3)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     let l2_cache = format!("L2 Cache: {}", lscpu_search(String::from("L2"), 10, false));
//     let l2_cache = &l2_cache[0..l2_cache.len()];
//     let l2_cache_label = Label::builder()
//         .label(l2_cache)
//         .margin_top(3)
//         .margin_bottom(3)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     let l3_cache = format!("L3 Cache: {}", lscpu_search(String::from("L3"), 10, false));
//     let l3_cache = &l3_cache[0..l3_cache.len()];
//     let l3_cache_label = Label::builder()
//         .label(l3_cache)
//         .margin_top(3)
//         .margin_bottom(3)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     let virtulisation = lscpu_search(String::from("(vt-x)|(amd-v)"), 15, true).to_lowercase();
//     let virt_enabled = if virtulisation == *"amd-v" || virtulisation == *"vt-x" {
//         "CPU Virtualisation: Enabled"
//     } else {
//         "CPU Virtualisation: Disabled"
//     };

//     let virt_enabled_label = Label::builder()
//         .label(virt_enabled)
//         .margin_top(3)
//         .margin_bottom(3)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     let cpu_info = Box::builder()
//         .orientation(Orientation::Vertical)
//         .margin_top(3)
//         .build();
//     cpu_info.append(&cpu_model_label);
//     cpu_info.append(&l1_cache_label);
//     cpu_info.append(&l2_cache_label);
//     cpu_info.append(&l3_cache_label);
//     cpu_info.append(&virt_enabled_label);

//     let used_memory_button = Button::builder()
//         .label("Press for used memory")
//         .margin_top(3)
//         .margin_bottom(3)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     used_memory_button.connect_clicked(move |used_memory_button| {
//         let string = format!("{}", System::new_all().used_memory());
//         let string = &string[..string.len()];
//         used_memory_button.set_label(string);
//     });

//     let full_box = Box::builder()
//         .orientation(Orientation::Vertical)
//         .margin_top(3)
//         .build();
//     full_box.append(&cpu_info);
//     full_box.append(&used_memory_button);

//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("Sysmontask-rs")
//         .child(&full_box)
//         .build();

//     window.show();
// }
