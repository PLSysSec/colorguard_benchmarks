use chrono::{prelude::*, Duration};
use serde::Serialize;
//use tera::{Context, Tera};
use std::time;
use std::thread::sleep;
use tinytemplate::TinyTemplate;
use std::error::Error;

use rand_distr::{Poisson, Distribution};
// fn delay_time() -> u64 {
//   1000
// }

fn delay() {
  // let t = delay_time();
  // let ten_millis = time::Duration::from_millis(10);
  // sleep(ten_millis);
  let lambda = 10.0;
  let poi = Poisson::new(lambda).unwrap();
  let v = poi.sample(&mut rand::thread_rng());
  let millis = time::Duration::from_millis(v as u64);
  sleep(millis);
}

#[derive(Serialize)]
struct Context {
    page_title: String,
    vendor_title: String,
    address: String,
    customer: String,
    payment: String,
    date_created: String,
    date_due: String,
    invoice_items: Vec<InvoiceItem>,
    total_price: String,
    invoice_num: String,
    css_style: String,
}


#[derive(Serialize)]
struct InvoiceItem {
    name: String,
    #[serde(serialize_with = "serialize_price")]
    price: f32,
}

fn serialize_price<S: serde::Serializer>(price: &f32, s: S) -> Result<S::Ok, S::Error> {
    let price_str = format!("{:.2}", price);
    s.serialize_str(&price_str)
}


// #[fastly::main]
// #[wasm_bindgen]
pub fn main() -> Result<(), Box<dyn Error>> {
  delay();

    // Invoice item data
    let invoice_items = vec![
        InvoiceItem {
            name: "Pet Food".into(),
            price: 19.99,
        },
        InvoiceItem {
            name: "Leash".into(),
            price: 10.75,
        },
        InvoiceItem {
            name: "Shampoo".into(),
            price: 9.99,
        },
    ];

    // Caculate total price from invoice items
    let total_price: f32 = invoice_items.iter().map(|item| item.price).sum();

    // Invoice create date and due date
    let now = Local::now();
    let due = now + Duration::days(30);
    let date_created = now.format("%b %e %Y").to_string();
    let date_due = due.format("%b %e %Y").to_string();

    // Create context object for template
    //let mut context = Context::new();
    let context = Context {
        
    

    // Set template data to context
    page_title: "Sample Invoice, powered by Fastly".to_string(),
    vendor_title: "Fastly Invoice".to_string(),
    address: "P.O. Box 78266<br/>San Francisco, CA 94107 ".to_string(),
    customer: "Example Corp.<br />billing@example.com".to_string(),
    payment: "Credit card".to_string(),
    date_created: date_created,
    date_due: date_due,
    invoice_items: invoice_items,
    total_price: format!("{:.2}", total_price),
    invoice_num: "10937248".to_string(),

    // Set template CSS style
    css_style: INVOICE_STYLE.to_string(),

    };

    let mut tt = TinyTemplate::new();
    tt.add_template("invoice", INVOICE_TEMPLATE)?;
    let rendered = tt.render("invoice", &context)?;
    

    // Render the template with context data
     /*
    let invoice = Tera::default()
        .render_str(INVOICE_TEMPLATE, &context)
        .unwrap();
*/
    // Send the template to client
    println!("{}", rendered);
    
    Ok(())
}

// Invoice HTML template, with template language script
// Credit to https://github.com/sparksuite/simple-html-invoice-template
const INVOICE_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>{page_title}</title>
    <style>{css_style}</style>
  </head>

  <body>
    <div class="invoice-box">
      <table cellpadding="0" cellspacing="0">
        <tr class="top">
          <td colspan="2">
            <table>
              <tr>
                <td class="title">{vendor_title}</td>
                <td>
                  Invoice #: {invoice_num}<br />
                  Created: {date_created}<br />
                  Due: {date_due}
                </td>
              </tr>
            </table>
          </td>
        </tr>

        <tr class="information">
          <td colspan="2">
            <table>
              <tr>
                <td>{address}</td>
                <td>{customer}</td>
              </tr>
            </table>
          </td>
        </tr>

        <tr class="heading"><td>Item</td><td>Price</td></tr>


        {{ for item in invoice_items }}
        <tr class="item">
          <td>{item.name}</td>
          <td>${item.price}</td>
        </tr>
        {{ endfor }}
    

        <tr class="total">
          <td></td>
          <td>Total: ${total_price}</td>
        </tr>
      </table>
    </div>
  </body>
</html>
"#;

// Invoice HTML style
const INVOICE_STYLE: &str = r#"
.invoice-box {
  max-width: 800px;
  margin: auto;
  padding: 30px;
  border: 1px solid #eee;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.15);
  font-size: 16px;
  line-height: 24px;
  font-family: 'Helvetica Neue', 'Helvetica', Helvetica, Arial, sans-serif;
  color: #555;
}

.invoice-box table {
  width: 100%;
  line-height: inherit;
  text-align: left;
}

.invoice-box table td {
  padding: 5px;
  vertical-align: top;
}

.invoice-box table tr td:nth-child(2) {
  text-align: right;
}

.invoice-box table tr.top table td {
  padding-bottom: 20px;
}

.invoice-box table tr.top table td.title {
  font-size: 45px;
  line-height: 45px;
  color: #333;
}

.invoice-box table tr.information table td {
  padding-bottom: 40px;
}

.invoice-box table tr.heading td {
  background: #eee;
  border-bottom: 1px solid #ddd;
  font-weight: bold;
}

.invoice-box table tr.details td {
  padding-bottom: 20px;
}

.invoice-box table tr.item td {
  border-bottom: 1px solid #eee;
}

.invoice-box table tr.item.last td {
  border-bottom: none;
}

.invoice-box table tr.total td:nth-child(2) {
  border-top: 2px solid #eee;
  font-weight: bold;
}

@media only screen and (max-width: 600px) {
  .invoice-box table tr.top table td {
    width: 100%;
    display: block;
    text-align: center;
  }

  .invoice-box table tr.information table td {
    width: 100%;
    display: block;
    text-align: center;
  }
}

/** RTL **/
.invoice-box.rtl {
  direction: rtl;
  font-family: Tahoma, 'Helvetica Neue', 'Helvetica', Helvetica, Arial, sans-serif;
}

.invoice-box.rtl table {
  text-align: right;
}

.invoice-box.rtl table tr td:nth-child(2) {
  text-align: left;
}
"#;
