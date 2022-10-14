use phf::phf_map;
use phf::phf_set;

const APOSTROPHE: &'static str = "'";

static INITIAL_POSITION_MAP: phf::Map<char, &'static str> = phf_map! {
    'А' => "A", 'а' => "a",
    'Б' => "B", 'б' => "b",
    'В' => "V", 'в' => "v",
    'Г' => "H", 'г' => "h",
    'Ґ' => "G", 'ґ' => "g",
    'Д' => "D", 'д' => "d",
    'Е' => "E", 'е' => "e",
    'Є' => "Ye", 'є' => "ye",
    'Ж' => "Zh", 'ж' => "zh",
    'З' => "Z", 'з' => "z",
    'И' => "Y", 'и' => "y",
    'І' => "I", 'і' => "i",
    'Ї' => "Yi", 'ї' => "yi",
    'Й' => "Y", 'й' => "y",
    'К' => "K", 'к' => "k",
    'Л' => "L", 'л' => "l",
    'М' => "M", 'м' => "m",
    'Н' => "N", 'н' => "n",
    'О' => "O", 'о' => "o",
    'П' => "P", 'п' => "p",
    'Р' => "R", 'р' => "r",
    'С' => "S", 'с' => "s",
    'Т' => "T", 'т' => "t",
    'У' => "U", 'у' => "u",
    'Ф' => "F", 'ф' => "f",
    'Х' => "Kh", 'х' => "kh",
    'Ц' => "Ts", 'ц' => "ts",
    'Ч' => "Ch", 'ч' => "ch",
    'Ш' => "Sh", 'ш' => "sh",
    'Щ' => "Shch", 'щ' => "shch",
    'Ю' => "Yu", 'ю' => "yu",
    'Я' => "Ya", 'я' => "ya"
};

static OTHER_POSITION_MAP: phf::Map<char, &'static str> = phf_map! {
    'А' => "A", 'а' => "a",
    'Б' => "B", 'б' => "b",
    'В' => "V", 'в' => "v",
    'Г' => "H", 'г' => "h",
    'Ґ' => "G", 'ґ' => "g",
    'Д' => "D", 'д' => "d",
    'Е' => "E", 'е' => "e",
    'Є' => "Ie", 'є' => "ie",
    'Ж' => "Zh", 'ж' => "zh",
    'З' => "Z", 'з' => "z",
    'И' => "Y", 'и' => "y",
    'І' => "I", 'і' => "i",
    'Ї' => "I", 'ї' => "i",
    'Й' => "I", 'й' => "i",
    'К' => "K", 'к' => "k",
    'Л' => "L", 'л' => "l",
    'М' => "M", 'м' => "m",
    'Н' => "N", 'н' => "n",
    'О' => "O", 'о' => "o",
    'П' => "P", 'п' => "p",
    'Р' => "R", 'р' => "r",
    'С' => "S", 'с' => "s",
    'Т' => "T", 'т' => "t",
    'У' => "U", 'у' => "u",
    'Ф' => "F", 'ф' => "f",
    'Х' => "Kh", 'х' => "kh",
    'Ц' => "Ts", 'ц' => "ts",
    'Ч' => "Ch", 'ч' => "ch",
    'Ш' => "Sh", 'ш' => "sh",
    'Щ' => "Shch", 'щ' => "shch",
    'Ю' => "Iu", 'ю' => "iu",
    'Я' => "Ia", 'я' => "ia"
};

static AFTER_APOSTROPHE_SET: phf::Set<char> = phf_set! {
    'я', 'Я',
    'ю', 'Ю',
    'є', 'Є',
    'ї', 'Ї'
};

fn is_non_initial_apostrophe(char: &char, initial: &bool) -> bool {
    *char == '\'' && !initial.clone()
}

/// Transliterates Ukrainian cyrillic text
/// by means of the Latin alphabet
///
/// # Examples
///
/// ```
/// let cyrillic: String = "Володимир".to_string();
/// let romanized: String = uaroman::romanize(cyrillic);
///
/// assert_eq!("Volodymyr", romanized);
/// ```
pub fn romanize(text: String) -> String {
    let mut initial: bool = true;
    let mut after_z: bool = false;
    let mut after_non_initial_apostrophe: bool = false;
    let mut utf8_char_buf: [u8; 4] = [0; 4];
    let mut romanized_char: String;
    let mut romanized_text: String = String::from("");

    for input_char in text.chars() {
        // restore held non-initial, non-diacritical apostrophe
        if after_non_initial_apostrophe && !AFTER_APOSTROPHE_SET.contains(&input_char) {
            romanized_text.push_str(APOSTROPHE);
        }
        if after_z && input_char == 'г' {
            // special case: "зг" is transliterated as "zgh"
            romanized_char = String::from("gh");
        } else if input_char == 'ь' || is_non_initial_apostrophe(&input_char, &initial) {
            // remove soft sign, hold non-initial apostrophe:
            // soft sign and diacritical apostrophe are not reproduced in Latin
            romanized_char = String::from("");
        } else {
            // map input character to output string, using distinct
            // maps for characters in initial and non-initial position
            let map = if initial { &INITIAL_POSITION_MAP } else { &OTHER_POSITION_MAP };
            romanized_char = match map.get(&input_char) {
                Some(output_str) => {
                    initial = false;
                    output_str.to_string()
                },
                _none => {
                    initial = true;
                    input_char.encode_utf8(&mut utf8_char_buf).to_string()
                }
            };
        }
        // set flags for next iteration
        after_z = input_char == 'З' || input_char == 'з';
        after_non_initial_apostrophe = is_non_initial_apostrophe(&input_char, &initial);
        // append output from current iteration to output text
        romanized_text.push_str(&romanized_char);
    }

    // if we held a trailing apostrophe, append it to final output text
    if after_non_initial_apostrophe {
        romanized_text.push_str(APOSTROPHE);
    }

    romanized_text
}

#[cfg(test)]
mod test {
    use super::*;

    fn convert_and_compare(cyrillic: &str, expected: &str) {
        let romanized = romanize(cyrillic.to_string());
        assert_eq!(romanized, expected);
    }

    #[test]
    fn apostrophes() {
        let cyrillic = "'Ярошенко' Згурський Знам'янка Ґорґани Згорани 'Розгон' Захар'їн";
        let expected = "'Yaroshenko' Zghurskyi Znamianka Gorgany Zghorany 'Rozghon' Zakharin";
        convert_and_compare(cyrillic, expected);
    }

    // examples from the paper submitted by Ukraine to
    // United Nations Group of Experts on Geographical Names
    // https://unstats.un.org/unsd/geoinfo/ungegn/docs/26th-gegn-docs/WP/WP21_Roma_system_Ukraine%20_engl._.pdf

    #[test]
    fn a1() {
        convert_and_compare("Алушта", "Alushta");
    }

    #[test]
    fn a2() {
        convert_and_compare("Андрій", "Andrii");
    }

    // error in pdf, last character in Latin word is cyrillic "а" -- corrected here
    #[test]
    fn b1() {
        convert_and_compare("Борщагівка", "Borshchahivka");
    }

    #[test]
    fn b2() {
        convert_and_compare("Борисенко", "Borysenko");
    }

    #[test]
    fn v1() {
        convert_and_compare("Вінниця", "Vinnytsia");
    }

    #[test]
    fn v2() {
        convert_and_compare("Володимир", "Volodymyr");
    }

    #[test]
    fn h1() {
        convert_and_compare("Гадяч", "Hadiach");
    }

    #[test]
    fn h2() {
        convert_and_compare("Богдан", "Bohdan");
    }

    #[test]
    fn h3() {
        convert_and_compare("Згурський", "Zghurskyi");
    }

    #[test]
    fn g1() {
        convert_and_compare("Ґалаґан", "Galagan");
    }

    #[test]
    fn g2() {
        convert_and_compare("Ґорґани", "Gorgany");
    }

    #[test]
    fn d1() {
        convert_and_compare("Донецьк", "Donetsk");
    }

    #[test]
    fn d2() {
        convert_and_compare("Дмитро", "Dmytro");
    }

    #[test]
    fn e1() {
        convert_and_compare("Рівне", "Rivne");
    }

    #[test]
    fn e2() {
        convert_and_compare("Олег", "Oleh");
    }

    #[test]
    fn e3() {
        convert_and_compare("Есмань", "Esman");
    }

    #[test]
    fn ye1() {
        convert_and_compare("Єнакієве", "Yenakiieve");
    }

    #[test]
    fn ye2() {
        convert_and_compare("Гаєвич", "Haievych");
    }

    #[test]
    fn ie1() {
        convert_and_compare("Короп'є", "Koropie");
    }

    #[test]
    fn zh1() {
        convert_and_compare("Житомир", "Zhytomyr");
    }

    #[test]
    fn zh2() {
        convert_and_compare("Жанна", "Zhanna");
    }

    #[test]
    fn zh3() {
        convert_and_compare("Жежелів", "Zhezheliv");
    }

    #[test]
    fn z1() {
        convert_and_compare("Закарпаття", "Zakarpattia");
    }

    #[test]
    fn z2() {
        convert_and_compare("Казимирчук", "Kazymyrchuk");
    }

    #[test]
    fn y1() {
        convert_and_compare("Медвин", "Medvyn");
    }

    #[test]
    fn y2() {
        convert_and_compare("Михайленко", "Mykhailenko");
    }

    #[test]
    fn i1() {
        convert_and_compare("Іванків", "Ivankiv");
    }

    #[test]
    fn i2() {
        convert_and_compare("Іващенко", "Ivashchenko");
    }

    #[test]
    fn yi1() {
        convert_and_compare("Їжакевич", "Yizhakevych");
    }

    #[test]
    fn yi2() {
        convert_and_compare("Кадиївка", "Kadyivka");
    }

    #[test]
    fn yi3() {
        convert_and_compare("Мар'їне", "Marine");
    }

    #[test]
    fn j1() {
        convert_and_compare("Йосипівка", "Yosypivka");
    }

    #[test]
    fn j2() {
        convert_and_compare("Стрий", "Stryi");
    }

    #[test]
    fn j3() {
        convert_and_compare("Олексій", "Oleksii");
    }

    #[test]
    fn k1() {
        convert_and_compare("Київ", "Kyiv");
    }

    #[test]
    fn k2() {
        convert_and_compare("Коваленко", "Kovalenko");
    }

    #[test]
    fn l1() {
        convert_and_compare("Лебедин", "Lebedyn");
    }

    #[test]
    fn l2() {
        convert_and_compare("Леонід", "Leonid");
    }

    #[test]
    fn m1() {
        convert_and_compare("Миколаїв", "Mykolaiv");
    }

    #[test]
    fn m2() {
        convert_and_compare("Маринич", "Marynych");
    }

    #[test]
    fn n1() {
        convert_and_compare("Ніжин", "Nizhyn");
    }

    // error in pdf, second "i" in Latin word is Ukrainian "і" -- corrected here
    #[test]
    fn n2() {
        convert_and_compare("Наталія", "Nataliia");
    }

    #[test]
    fn o1() {
        convert_and_compare("Одеса", "Odesa");
    }

    #[test]
    fn o2() {
        convert_and_compare("Онищенко", "Onyshchenko");
    }

    #[test]
    fn p1() {
        convert_and_compare("Полтава", "Poltava");
    }

    #[test]
    fn p2() {
        convert_and_compare("Петро", "Petro");
    }

    #[test]
    fn r1() {
        convert_and_compare("Решетилівка", "Reshetylivka");
    }

    #[test]
    fn r2() {
        convert_and_compare("Рибчинський", "Rybchynskyi");
    }

    #[test]
    fn s1() {
        convert_and_compare("Суми", "Sumy");
    }

    #[test]
    fn s2() {
        convert_and_compare("Соломія", "Solomiia");
    }

    #[test]
    fn t1() {
        convert_and_compare("Тернопіль", "Ternopil");
    }

    #[test]
    fn t2() {
        convert_and_compare("Троць", "Trots");
    }

    #[test]
    fn u1() {
        convert_and_compare("Ужгород", "Uzhhorod");
    }

    #[test]
    fn u2() {
        convert_and_compare("Уляна", "Uliana");
    }

    #[test]
    fn f1() {
        convert_and_compare("Фастів", "Fastiv");
    }

    #[test]
    fn f2() {
        convert_and_compare("Філіпчук", "Filipchuk");
    }

    #[test]
    fn kh1() {
        convert_and_compare("Харків", "Kharkiv");
    }

    #[test]
    fn kh2() {
        convert_and_compare("Христина", "Khrystyna");
    }

    #[test]
    fn ts1() {
        convert_and_compare("Біла Церква", "Bila Tserkva");
    }

    #[test]
    fn ts2() {
        convert_and_compare("Стеценко", "Stetsenko");
    }

    #[test]
    fn ch1() {
        convert_and_compare("Чернівці", "Chernivtsi");
    }

    #[test]
    fn ch2() {
        convert_and_compare("Шевченко", "Shevchenko");
    }

    #[test]
    fn sh1() {
        convert_and_compare("Шостка", "Shostka");
    }

    #[test]
    fn sh2() {
        convert_and_compare("Кишеньки", "Kyshenky");
    }

    #[test]
    fn shch1() {
        convert_and_compare("Щербухи", "Shcherbukhy");
    }

    #[test]
    fn shch2() {
        convert_and_compare("Гоща", "Hoshcha");
    }

    #[test]
    fn shch3() {
        convert_and_compare("Гаращенко", "Harashchenko");
    }

    #[test]
    fn yu1() {
        convert_and_compare("Юрій", "Yurii");
    }

    #[test]
    fn iu1() {
        convert_and_compare("Корюківка", "Koriukivka");
    }

    #[test]
    fn ya1() {
        convert_and_compare("Яготин", "Yahotyn");
    }

    #[test]
    fn ya2() {
        convert_and_compare("Ярошенко", "Yaroshenko");
    }

    #[test]
    fn ia1() {
        convert_and_compare("Костянтин", "Kostiantyn");
    }

    #[test]
    fn ia2() {
        convert_and_compare("Знам'янка", "Znamianka");
    }

    #[test]
    fn ia3() {
        convert_and_compare("Феодосія", "Feodosiia");
    }
}
