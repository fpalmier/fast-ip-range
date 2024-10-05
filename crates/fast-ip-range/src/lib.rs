use neon::prelude::*;
use std::cell::RefCell;

use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use iprange::IpRange;
use std::net::IpAddr;

type BoxedDatabase = JsBox<RefCell<Database>>;

struct Database {
    ip_v4: IpRange<Ipv4Net>,
    ip_v6: IpRange<Ipv6Net>,
}

impl Finalize for Database {}

impl Database {
    fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<RefCell<Database>>> {
        let db = RefCell::new(Database {
            ip_v4: IpRange::default(),
            ip_v6: IpRange::default(),
        });

        Ok(cx.boxed(db))
    }

    fn js_add(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let db = cx.argument::<BoxedDatabase>(0)?;
        let mut db = db.borrow_mut();

        let net = cx.argument::<JsString>(1)?.value(&mut cx);
        let net: IpNet = match net.parse() {
            Ok(net) => net,
            Err(_) => return cx.throw_error(format!("invalid Net address syntax: {net}")),
        };

        match net {
            IpNet::V4(net) => {
                db.ip_v4.add(net);
            }
            IpNet::V6(net) => {
                db.ip_v6.add(net);
            }
        }

        Ok(cx.undefined())
    }

    fn js_contains(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let db = cx.argument::<BoxedDatabase>(0)?;
        let db = db.borrow();

        let ip = cx.argument::<JsString>(1)?.value(&mut cx);
        let ip: IpAddr = match ip.parse() {
            Ok(ip) => ip,
            Err(_) => return cx.throw_error(format!("invalid ip address syntax: {ip}")),
        };

        match ip {
            IpAddr::V4(ip) => {
                let contains = db.ip_v4.contains(&ip);
                Ok(cx.boolean(contains))
            }
            IpAddr::V6(ip) => {
                let contains = db.ip_v6.contains(&ip);
                Ok(cx.boolean(contains))
            }
        }
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("databaseNew", Database::js_new)?;
    cx.export_function("databaseAdd", Database::js_add)?;
    cx.export_function("databaseContains", Database::js_contains)?;

    Ok(())
}
