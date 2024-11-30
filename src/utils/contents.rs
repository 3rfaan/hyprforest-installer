use colored::Colorize;
use std::collections::BTreeMap;

pub fn print_installer_info() {
    const LOGO: &str = "

██╗  ██╗██╗   ██╗██████╗ ██████╗ ███████╗ ██████╗ ██████╗ ███████╗███████╗████████╗
██║  ██║╚██╗ ██╔╝██╔══██╗██╔══██╗██╔════╝██╔═══██╗██╔══██╗██╔════╝██╔════╝╚══██╔══╝
███████║ ╚████╔╝ ██████╔╝██████╔╝█████╗  ██║   ██║██████╔╝█████╗  ███████╗   ██║   
██╔══██║  ╚██╔╝  ██╔═══╝ ██╔══██╗██╔══╝  ██║   ██║██╔══██╗██╔══╝  ╚════██║   ██║   
██║  ██║   ██║   ██║     ██║  ██║██║     ╚██████╔╝██║  ██║███████╗███████║   ██║   
╚═╝  ╚═╝   ╚═╝   ╚═╝     ╚═╝  ╚═╝╚═╝      ╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   

";
    const TITLE: &str = "Auto-ricer for Hyprland on Arch Linux";
    const AUTHOR: &str = "Arfan Zubi";
    const THEME: &str = "Catppuccin";
    const LICENSE: &str = "GNU General Public License";

    println!(
        "{logo}\n\
        {title}\n\
        {} {author}\n\
        {} {theme}\n\
        {} {license}\n",
        "Author:".yellow(),
        "Theme:".bright_black(),
        "License:".bright_black(),
        logo = LOGO.bright_black(),
        title = TITLE.to_uppercase().green().bold(),
        author = AUTHOR.yellow(),
        theme = THEME.bright_black(),
        license = LICENSE.bright_black()
    );
}

pub fn get_kb_layouts() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("al", "Albanian"),
        ("et", "Amharic"),
        ("am", "Armenian"),
        ("ara", "Arabic"),
        ("eg", "Arabic (Egypt)"),
        ("iq", "Arabic (Iraq)"),
        ("ma", "Arabic (Morocco)"),
        ("sy", "Arabic (Syria)"),
        ("az", "Azerbaijani"),
        ("ml", "Bambara"),
        ("bd", "Bangla"),
        ("by", "Belarusian"),
        ("be", "Belgian"),
        ("dz", "Berber (Algeria, Latin)"),
        ("ba", "Bosnian"),
        ("brai", "Braille"),
        ("bg", "Bulgarian"),
        ("mm", "Burmese"),
        ("cn", "Chinese"),
        ("hr", "Croatian"),
        ("cz", "Czech"),
        ("dk", "Danish"),
        ("af", "Dari"),
        ("mv", "Dhivehi"),
        ("nl", "Dutch"),
        ("bt", "Dzongkha"),
        ("au", "English (Australia)"),
        ("cm", "English (Cameroon)"),
        ("gh", "English (Ghana)"),
        ("nz", "English (New Zealand)"),
        ("ng", "English (Nigeria)"),
        ("za", "English (South Africa)"),
        ("gb", "English (UK)"),
        ("us", "English (US)"),
        ("epo", "Esperanto"),
        ("ee", "Estonian"),
        ("fo", "Faroese"),
        ("ph", "Filipino"),
        ("fi", "Finnish"),
        ("fr", "French"),
        ("ca", "French (Canada)"),
        ("cd", "French (Democratic Republic of the Congo)"),
        ("tg", "French (Togo)"),
        ("ge", "Georgian"),
        ("de", "German"),
        ("at", "German (Austria)"),
        ("ch", "German (Switzerland)"),
        ("gr", "Greek"),
        ("il", "Hebrew"),
        ("hu", "Hungarian"),
        ("is", "Icelandic"),
        ("in", "Indian"),
        ("id", "Indonesian (Latin)"),
        ("ie", "Irish"),
        ("it", "Italian"),
        ("jp", "Japanese"),
        ("kz", "Kazakh"),
        ("kh", "Khmer (Cambodia)"),
        ("kr", "Korean"),
        ("kg", "Kyrgyz"),
        ("la", "Lao"),
        ("lv", "Latvian"),
        ("lt", "Lithuanian"),
        ("mk", "Macedonian"),
        ("my", "Malay (Jawi, Arabic Keyboard)"),
        ("mt", "Maltese"),
        ("md", "Moldavian"),
        ("mn", "Mongolian"),
        ("me", "Montenegrin"),
        ("np", "Nepali"),
        ("gn", "N'Ko (AZERTY)"),
        ("no", "Norwegian"),
        ("ir", "Persian"),
        ("pl", "Polish"),
        ("pt", "Portuguese"),
        ("br", "Portuguese (Brazil)"),
        ("ro", "Romanian"),
        ("ru", "Russian"),
        ("rs", "Serbian"),
        ("lk", "Sinhala (phonetic)"),
        ("sk", "Slovak"),
        ("si", "Slovenian"),
        ("es", "Spanish"),
        ("latam", "Spanish (Latin American)"),
        ("ke", "Swahili (Kenya)"),
        ("tz", "Swahili (Tanzania)"),
        ("se", "Swedish"),
        ("tw", "Taiwanese"),
        ("tj", "Tajik"),
        ("th", "Thai"),
        ("bw", "Tswana"),
        ("tm", "Turkmen"),
        ("tr", "Turkish"),
        ("ua", "Ukrainian"),
        ("pk", "Urdu (Pakistan)"),
        ("uz", "Uzbek"),
        ("vn", "Vietnamese"),
        ("sn", "Wolof"),
    ])
}
