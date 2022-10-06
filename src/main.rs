use chrono::Utc;
use cron::Schedule;
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::{sleep, Duration};
use yaml_rust::YamlLoader;

#[tokio::main]
async fn main() {
    let mut arquivo = File::open("config.yml")
        .await
        .expect("Não consegui ser o arquivo!");
    let mut docs = String::new();
    arquivo
        .read_to_string(&mut docs)
        .await
        .expect("Falha ao converter");

    let data = YamlLoader::load_from_str(&docs).unwrap();
    let doc = &data[0];

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, doc["Webhook_URL"].as_str().unwrap())
        .await
        .expect("Falha ao configurar o Webhook");

    let schedule = Schedule::from_str("0 0 8 * * *").unwrap();

    println!("Alerta de Aniverssário ativado!");

    loop {
        let now = Utc::now();
        let job = schedule.upcoming(Utc).next().unwrap();
        let c = job.timestamp() as f64 - now.timestamp() as f64;
        let d = Duration::from_secs_f64(c);

        let v = job.to_string();
        let v1 = v.split_whitespace().next().unwrap();
        let v2 = v1.split("-").collect::<Vec<&str>>();

        sleep(d).await;

        if !doc["Bdays"][v2[1]][v2[2]].is_badvalue() {
            println!("Dia de Festa!!");
            for i in doc["Bdays"][v2[1]][v2[2]].as_vec().unwrap() {
                let s = i.as_str().unwrap();
                let cont = format!("Bom dia, @everyone!\nHoje Vamos dar os parabêns ao <@{}> pelo seu Aniversário! 🎉", s);

                webhook
                    .execute(&http, false, |w| w.content(cont).username("B-Day Staff"))
                    .await
                    .expect("Falha ao enviar o webhook");
            }
        } else {
            println!("Mais um dia Normal!");
        }
    }
}
