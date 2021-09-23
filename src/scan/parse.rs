use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Parseip {
	pub ip: String,
    pub port: String,
    pub timeout: u64,
    pub udp: bool, 
    pub concurrency: u32,
    pub outfile: Option<PathBuf>,
}

impl Parseip {
    pub async fn get_ips(&self) -> Result<Vec<String>,()> {
        // 判断是否存在 - 10.1-244.0.1
        let idx1 = match self.ip.find("-") {
            Some(idx) => idx,
            None => 0,
        };

        if idx1 == 0 {
            return Ok(vec![format!("{}", self.ip)])
        }

        let mut a = Vec::new();
		let mut b = Vec::new();
		let mut c = Vec::new();
		let mut d = Vec::new();

        let tmp: Vec<&str> = self.ip.split(".").collect();
        for (index,n) in tmp.iter().enumerate() {
            if index == 0 {
				a = self.strtoi32(n);
			} else if index == 1 {
				b = self.strtoi32(n)
			} else if index == 2 {
				c = self.strtoi32(n)
			} else if index == 3 {
				d = self.strtoi32(n)
			}
        }

        let data = &mut Vec::new();
        for x in a.clone() {
            for y in b.clone() {
                for z in c.clone() {
                    for g in d.clone() {
                        data.push(format!("{}.{}.{}.{}",x,y,z,g))
                    }
                }
            }
        }
            
        Ok(data.to_vec())
    }

    // 1-3 -> [1,2,3]
    fn strtoi32(&self, input: &str) -> Vec<i32> {
        let mut v = Vec::new();
		if input.contains("-") {
			// Vec<str> -> Vec<i32> -> for x in a..b
			let tmp:Vec<i32> = input.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
			for x in tmp[0]..tmp[1] {
				v.push(x);
			}
		} else {
			v.push(input.parse::<i32>().unwrap());
		}
		v
    }

    pub async fn get_ports(&self) -> Result<Vec<String>,()> {
        let idx1 = match self.port.find("-") {
            Some(idx) => idx,
            None => 0,
        };

        let idx2 = match self.port.find(",") {
            Some(idx) => idx,
            None => 0,
        };

        if idx1 == 0 && idx2 == 0 {
            return match self.port.parse::<i32>() {
                Ok(i) => {
                    Ok(vec![format!("{}", i)])
                }
                Err(_) => {
                    Err(())
                }
            };
        }
        let mut lists = Vec::new();

        // 处理情况: 80,84,86-89,59
        if idx1 != 0 && idx2 != 0 {
            let sli: Vec<&str> = self.port.split(",").collect();
            for i in sli {
                if let Some(t) = i.find("-") {
                    let start = i[..t].parse::<i32>().unwrap();
                    let end = i[t + 1..].parse::<i32>().unwrap();
                    for ic in start..=end {
                        lists.push(format!("{}", ic));
                    }
                } else {
                    lists.push(i.trim().to_string());
                }
            }

            return Ok(lists);
        }

        // param1
        if idx1 != 0 {
            let start = *&self.port[..idx1].parse::<i32>().unwrap();
            let end = *&self.port[idx1 + 1..].parse::<i32>().unwrap();
            for i in start..=end {
                lists.push(format!("{}", i));
            }
            return Ok(lists);
        }

        // param2
        let sli: Vec<&str> = self.port.split(",").collect();
        for i in sli {
            lists.push(i.trim().to_string());
        }

        Ok(lists)
    }
}