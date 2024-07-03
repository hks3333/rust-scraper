fn main() {
let response = reqwest::blocking::get("https://crypto.com/price",)
.unwrap().text().unwrap();
let document = scraper::Html::parse_document(&response);
let cryp_selector = scraper::Selector::parse("div.css-ttxvk0>p").unwrap();
let cryps = document.select(&cryp_selector).map(|x| x.inner_html());
let price_selector = scraper::Selector::parse("div.css-16q9pr7>div").unwrap();
let price = document.select(&price_selector).map(|y| y.inner_html());
let change_selector = scraper::Selector::parse("td.css-1b7j986>p").unwrap();
let change = document.select(&change_selector).map(|z| z.inner_html());
let volume_selector = scraper::Selector::parse("td.css-1nh9lk8").unwrap();
let sel = document.select(&volume_selector).map(|k| k.inner_html());
let mut crypname: Vec<String> = vec![String::new(); 0];
let mut crypprice: Vec<String> = vec![String::new(); 0];
let mut change24: Vec<String> = vec![String::new(); 0];
let mut volume: Vec<String> = vec![String::new(); 0];
let mut cap: Vec<String> = vec![String::new(); 0];

for i in cryps
{
crypname.push(i);
}
for i in price
{
crypprice.push(i);
}
for i in change
{
change24.push(i);
}

let mut i=0;
for val in sel
{
i+=1;
if i%2==0
{
cap.push(val);
}
else
{
volume.push(val);
}
}


let mut wtr=csv::Writer::from_path("cryptocurrency.csv").unwrap();
wtr.write_record(&["Name","Price","24 Hour Change","24 Hour Volume","Market Cap"]).unwrap();
for x in 0..50
{
wtr.write_record([&crypname[x],&crypprice[x],&change24[x],&volume[x],&cap[x]]);
wtr.flush();
}
println!("Done")
}


