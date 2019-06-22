use std::fs;
use std::io::Read;
// use std::io::Write;
use std::path::Path;

use crate::task::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Puzzle {
    pub b_num: usize,
    pub e_num: usize,
    pub t_size: usize,
    pub v_min: usize,
    pub v_max: usize,

    pub booster_num: Vec<usize>,

    pub i_seq: Vec<Point>,
    pub o_seq: Vec<Point>,
}

impl Puzzle {
    pub fn from(s: &str) -> Puzzle {
        let s = s.trim_end().split('#').collect::<Vec<_>>();
        let nums = s[0].split(",").collect::<Vec<_>>();
        let b_num = nums[0].parse::<usize>().expect("failed to parse num");
        let e_num = nums[1].parse::<usize>().expect("failed to parse num");
        let t_size = nums[2].parse::<usize>().expect("failed to parse num");
        let v_min = nums[3].parse::<usize>().expect("failed to parse num");
        let v_max = nums[4].parse::<usize>().expect("failed to parse num");
        let m_num = nums[5].parse::<usize>().expect("failed to parse num");
        let f_num = nums[6].parse::<usize>().expect("failed to parse num");
        let d_num = nums[7].parse::<usize>().expect("failed to parse num");
        let r_num = nums[8].parse::<usize>().expect("failed to parse num");
        let c_num = nums[9].parse::<usize>().expect("failed to parse num");
        let x_num = nums[10].parse::<usize>().expect("failed to parse num");
        let Map(i_seq) = Map::from(s[1]);
        let Map(o_seq) = Map::from(s[2]);
        Puzzle {
            b_num,
            e_num,
            t_size,
            v_min,
            v_max,
            booster_num: vec![m_num, f_num, d_num, x_num, r_num, c_num],
            i_seq,
            o_seq,
        }
    }
}

#[test]
fn test_puzzle() {
    let s = "1,1,150,400,1200,6,10,5,1,3,4#(73,61),(49,125),(73,110),(98,49),(126,89),(68,102),(51,132),(101,123),(22,132),(71,120),(97,129),(118,76),(85,100),(88,22),(84,144),(93,110),(96,93),(113,138),(91,52),(27,128),(84,140),(93,143),(83,17),(123,85),(50,74),(139,97),(101,110),(77,56),(86,23),(117,59),(133,126),(83,135),(76,90),(70,12),(12,141),(116,87),(102,76),(19,138),(86,129),(86,128),(83,60),(100,98),(60,105),(61,103),(94,99),(130,124),(141,132),(68,84),(86,143),(72,119)#(145,82),(20,65),(138,99),(38,137),(85,8),(125,104),(117,48),(57,48),(64,119),(3,25),(40,22),(82,54),(121,119),(1,34),(43,98),(97,120),(10,90),(15,32),(41,13),(86,40),(3,83),(2,127),(4,40),(139,18),(96,49),(53,22),(5,103),(112,33),(38,47),(16,121),(133,99),(113,45),(50,5),(94,144),(16,0),(93,113),(18,141),(36,25),(56,120),(3,126),(143,144),(99,62),(144,117),(48,97),(69,9),(0,9),(141,16),(55,68),(81,3),(47,53)";
    Puzzle::from(&s);
}