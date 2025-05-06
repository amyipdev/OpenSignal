use crate::workbench;
use adw::prelude::*;
use image::ImageEncoder;
use std::io::{BufRead, Write};
use std::rc::Rc;
use std::sync::{Arc, LazyLock, Mutex};

const QR_RESO: u32 = 320;

static SIGNAL_ID: LazyLock<Arc<Mutex<String>>> =
    LazyLock::new(|| Arc::new(Mutex::new(String::new())));

pub fn main() {
    gtk::init().unwrap_or(());
    settings();
    connections();
}

fn dependency_checker() -> bool {
    let refr: gtk::Button = workbench::builder().object("welcome-refr").unwrap();
    let cont: gtk::Button = workbench::builder().object("welcome-cont").unwrap();
    let misdeps: gtk::Box = workbench::builder().object("missing_dependencies").unwrap();
    misdeps.set_visible(false);
    refr.set_sensitive(false);
    refr.set_sensitive(false);
    let mut missing: bool = false;
    let l1: gtk::Label = workbench::builder().object("lcheck_sd").unwrap();
    let i1: gtk::Image = workbench::builder().object("icheck_sd").unwrap();
    let l2: gtk::Label = workbench::builder().object("lcheck_sc").unwrap();
    let i2: gtk::Image = workbench::builder().object("icheck_sc").unwrap();
    if !std::path::Path::new(&shellexpand::tilde("~/.config/Signal").to_string()).is_dir() {
        l1.set_label("Missing");
        i1.set_from_icon_name(Some("software-update-urgent-symbolic"));
        missing = true;
    } else {
        l1.set_label("Present");
        i1.set_from_icon_name(Some("emblem-ok-symbolic"));
    }
    if let Ok(_) = which::which("signal-cli") {
        l2.set_label("Present");
        i2.set_from_icon_name(Some("emblem-ok-symbolic"));
    } else {
        l2.set_label("Missing");
        i2.set_from_icon_name(Some("software-update-urgent-symbolic"));
        missing = true;
    }
    if missing {
        misdeps.set_visible(true);
        refr.set_sensitive(true);
        return false;
    } else {
        cont.set_sensitive(true);
        return true;
    }
}

fn connections() {
    let s: Rc<gtk::Stack> = Rc::new(workbench::builder().object("mainstack").unwrap());
    let s1 = s.clone();
    let s2 = s.clone();

    let welcome_gs: gtk::Button = workbench::builder().object("welcome-gs").unwrap();
    welcome_gs.connect_clicked(move |_| s1.set_visible_child_name("prerequisites"));
    let con: gtk::Button = workbench::builder().object("welcome-cont").unwrap();
    con.connect_clicked(move |_| s2.set_visible_child_name("connectpage"));
    let cag: gtk::Button = workbench::builder().object("welcome-refr").unwrap();
    cag.connect_clicked(|_| {
        dependency_checker();
    });
    let cont: gtk::Button = workbench::builder().object("linkcont").unwrap();
    cont.connect_clicked(|_| store_signal_user_id());
    let ltsn: gtk::Button = workbench::builder().object("linkcli").unwrap();
    ltsn.connect_clicked(|_| run_signal_cli_link());
}

fn store_signal_user_id() {
    let s: gtk::Stack = workbench::builder().object("mainstack").unwrap();
    s.set_visible_child_name("main_interface");
    launch_main();
    let pnent: gtk::Entry = workbench::builder().object("pnent").unwrap();
    let mut tab: toml::Table = toml::Table::new();
    let st = pnent.text().to_string();
    let sida: Arc<Mutex<String>> = Arc::clone(&SIGNAL_ID);
    {
        let mut sid = sida.lock().unwrap();
        *sid = st.clone();
    }
    tab.insert("sid".to_string(), toml::Value::String(st));
    std::fs::File::create(shellexpand::tilde("~/.config/autosignal/config.toml").to_string())
        .unwrap()
        .write_all(toml::to_string(&tab).unwrap().as_bytes());
}

fn run_signal_cli_link() {
    let mut child = std::process::Command::new("signal-cli")
        .arg("link")
        .arg("-n")
        .arg("OpenSignal via signal-cli")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let sor = std::io::BufReader::new(child.stdout.take().unwrap());
    let link = sor.lines().filter_map(|l| l.ok()).next().unwrap();
    let qrcode = qrc::QRCode::new(link.as_bytes().to_vec());
    let im: gtk::Image = workbench::builder().object("qrcode").unwrap();
    let load = gtk::gdk_pixbuf::PixbufLoader::with_type("png").unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let enc = image::codecs::png::PngEncoder::new(&mut buf);
    enc.write_image(
        &qrcode.to_png(QR_RESO),
        QR_RESO,
        QR_RESO,
        image::ExtendedColorType::Rgba8,
    );
    load.write(&buf).unwrap();
    load.close();
    im.set_from_pixbuf(Some(&load.pixbuf().unwrap()));
    let s: gtk::Stack = workbench::builder().object("mainstack").unwrap();
    s.set_visible_child_name("signal_cli_linker");
    std::thread::spawn(move || {
        if child.wait().unwrap().success() {
            store_signal_user_id();
        } else {
            std::thread::spawn(run_signal_cli_link);
            let tolbl: gtk::Label = workbench::builder().object("timed_out").unwrap();
            tolbl.set_visible(true);
        }
    });
}

fn settings() {
    let bp = shellexpand::tilde("~/.config/autosignal/").to_string();
    let pb = std::path::Path::new(&bp);
    std::fs::create_dir_all(&pb);
    let dcr = dependency_checker();
    let cnfp = pb.join("config.toml");
    if dcr && std::fs::exists(&cnfp).unwrap() {
        let config = std::fs::read_to_string(&cnfp)
            .unwrap()
            .parse::<toml::Table>()
            .unwrap();
        let sida: Arc<Mutex<String>> = Arc::clone(&SIGNAL_ID);
        {
            let mut sid = sida.lock().unwrap();
            *sid = config["sid"].as_str().unwrap().to_string();
        }
        let s: gtk::Stack = workbench::builder().object("mainstack").unwrap();
        s.set_visible_child_name("main_interface");
        launch_main();
    }
}

#[derive(Debug, Clone)]
struct Conversation {
    id: String,
    name: String,
    members: Vec<String>,
}

fn launch_main() {
    let k: String = serde_json::from_str::<serde_json::Value>(
        &std::fs::read_to_string(shellexpand::tilde("~/.config/Signal/config.json").to_string())
            .unwrap(),
    )
    .unwrap()["key"]
        .as_str()
        .unwrap()
        .to_string();
    let conn = rusqlite::Connection::open(
        shellexpand::tilde("~/.config/Signal/sql/db.sqlite").to_string(),
    )
    .unwrap();
    // This is not a vulnerability - no privilege escalation, and if something is
    // going to be able to write to your Signal config file, it can also read
    // the database and execute any command on it.
    // Group chat names and UUIDs
    let v = format!("PRAGMA key=\"x'{}'\";", k);
    conn.query_row(&v, (), |x| Ok(())).unwrap();

    let mut sql1 = conn
        .prepare("SELECT id, name, members FROM conversations;")
        .unwrap();
    let gi = sql1
        .query_map([], |row| {
            Ok(Conversation {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap_or("".to_string()),
                members: row
                    .get::<usize, String>(2)
                    .unwrap_or("".to_string())
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.to_string())
                    .collect(),
            })
        })
        .unwrap();
    let mut groups: Vec<Conversation> = gi
        .map(|x| x.unwrap())
        .filter(|x| x.name != "" && x.members.len() != 0)
        .collect();
    groups.sort_by_key(|x| x.name.clone().to_lowercase());
    let dd: gtk::DropDown = workbench::builder().object("chats-drop-down").unwrap();
    let mut names = Rc::new(gtk::StringList::new(
        &groups
            .iter()
            .map(|x| x.name.as_str())
            .collect::<Vec<&str>>(),
    ));
    dd.set_model(Some(names.clone().upcast_ref::<gtk::gio::ListModel>()));
    let send: gtk::Button = workbench::builder().object("send").unwrap();
    send.connect_clicked(move |_| send_message(groups.clone()));
}

fn send_message(groups: Vec<Conversation>) {
    let send: gtk::Button = workbench::builder().object("send").unwrap();
    send.set_sensitive(false);

    let dd: gtk::DropDown = workbench::builder().object("chats-drop-down").unwrap();
    let csgc = dd
        .model()
        .unwrap()
        .downcast_ref::<gtk::StringList>()
        .unwrap()
        .string(dd.selected())
        .unwrap();
    let dele: gtk::Entry = workbench::builder().object("msg-delay").unwrap();
    let del = dele.text().parse::<u64>().unwrap_or(3000);
    let tent: gtk::TextView = workbench::builder().object("msgtxt").unwrap();
    let tbuf = tent.buffer();
    let text = tbuf
        .text(&tbuf.start_iter(), &tbuf.end_iter(), false)
        .to_string();

    let pbar: gtk::ProgressBar = workbench::builder().object("pbar").unwrap();
    let group = groups.iter().find(|x| x.name == csgc).unwrap();
    let length = group.members.len() as f64;
    pbar.set_fraction(0.0);
    pbar.set_visible(true);

    let mbs = group.members.clone();
    std::thread::spawn(move || {
        let mut sid: String;
        let sida: Arc<Mutex<String>> = Arc::clone(&SIGNAL_ID);
        {
            sid = sida.lock().unwrap().clone();
        }

        let pbar1: gtk::ProgressBar = workbench::builder().object("pbar").unwrap();
        let send1: gtk::Button = workbench::builder().object("send").unwrap();
        for (i, m) in mbs.iter().enumerate() {
            std::thread::sleep(std::time::Duration::from_millis(del));
            std::process::Command::new("signal-cli")
                .arg("-u")
                .arg(sid.as_str())
                .arg("send")
                .arg("-m")
                .arg(text.as_str())
                .arg(m)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            pbar1.set_fraction(i as f64 / length);
        }
        pbar1.set_fraction(1.0);
        send1.set_sensitive(true);
    });
}
