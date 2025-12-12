use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Kullanım: lale <dosya.lale>");
        println!("Örnek: lale examples/merhaba.lale");
        return;
    }

    let dosya_yolu = &args[1];
    let içerik = fs::read_to_string(dosya_yolu)
        .expect("Dosya okunamadı");

    println!("Lale derleyicisi çalışıyor!");
    println!("Dosya: {}", dosya_yolu);
    println!("Satır sayısı: {}", içerik.lines().count());
    println!("\nİçerik:\n{}", içerik);

    // Şimdilik sadece token'lara bakıyoruz
    let tokenler = lale::tokenize(&içerik);
    println!("\nToken sayısı: {}", tokenler.len());
    println!("İlk 20 token: {:?}", &tokenler[..tokenler.len().min(20)]);

    println!("\nDerleme başarılı! (Henüz çıktı üretmiyoruz ama çok yakında! 🌷");
}
