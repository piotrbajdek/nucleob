// NUCLEOB VERSION 1.0.0 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE NUCLEOBASES

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::similar_names, clippy::too_many_lines)]

// FUNCTION

pub fn n_count(data: &str) {
    let reset = "\x1b[0m";
    let yellow = "\x1b[93m";
    let red = "\x1b[31m";
    let cyan = "\x1b[36m";

    // TOO MANY SEQUENCES

    let mut seq_no = 0;
    for line in data.split('\n') {
        if line.starts_with('>') {
            seq_no += 1;
            if seq_no == 2 {
                panic!("{}", red.to_owned() + "Too many sequences! Only one sequence per file is allowed." + reset);
            }
        }
    }

    // NUCLEOBASES

    for line in data.split('\n') {
        if line.starts_with('>') {
            let ade_rst: f32 = line.matches('A').count() as f32; // Adenine
            let ade_tot: f32 = data.matches('A').count() as f32;
            let ade_seq: f32 = ade_tot - ade_rst;

            let cyt_rst: f32 = line.matches('C').count() as f32; // Cytosine
            let cyt_tot: f32 = data.matches('C').count() as f32;
            let cyt_seq: f32 = cyt_tot - cyt_rst;

            let gua_rst: f32 = line.matches('G').count() as f32; // Guanine
            let gua_tot: f32 = data.matches('G').count() as f32;
            let gua_seq: f32 = gua_tot - gua_rst;

            let thy_rst: f32 = line.matches('T').count() as f32; // Thymine
            let thy_tot: f32 = data.matches('T').count() as f32;
            let thy_seq: f32 = thy_tot - thy_rst;

            let ura_rst: f32 = line.matches('U').count() as f32; // Uracil
            let ura_tot: f32 = data.matches('U').count() as f32;
            let ura_seq: f32 = ura_tot - ura_rst;

            // AMBIGUOUS

            let w_rst: f32 = line.matches('W').count() as f32; // weak
            let w_tot: f32 = data.matches('W').count() as f32;
            let w_seq: f32 = w_tot - w_rst;

            let s_rst: f32 = line.matches('S').count() as f32; // strong
            let s_tot: f32 = data.matches('S').count() as f32;
            let s_seq: f32 = s_tot - s_rst;

            let m_rst: f32 = line.matches('M').count() as f32; // amino
            let m_tot: f32 = data.matches('M').count() as f32;
            let m_seq: f32 = m_tot - m_rst;

            let k_rst: f32 = line.matches('K').count() as f32; // keto
            let k_tot: f32 = data.matches('K').count() as f32;
            let k_seq: f32 = k_tot - k_rst;

            let r_rst: f32 = line.matches('R').count() as f32; // purine
            let r_tot: f32 = data.matches('R').count() as f32;
            let r_seq: f32 = r_tot - r_rst;

            let y_rst: f32 = line.matches('Y').count() as f32; // pyrimidine
            let y_tot: f32 = data.matches('Y').count() as f32;
            let y_seq: f32 = y_tot - y_rst;

            let b_rst: f32 = line.matches('B').count() as f32; // not A
            let b_tot: f32 = data.matches('B').count() as f32;
            let b_seq: f32 = b_tot - b_rst;

            let d_rst: f32 = line.matches('D').count() as f32; // not C
            let d_tot: f32 = data.matches('D').count() as f32;
            let d_seq: f32 = d_tot - d_rst;

            let h_rst: f32 = line.matches('H').count() as f32; // not G
            let h_tot: f32 = data.matches('H').count() as f32;
            let h_seq: f32 = h_tot - h_rst;

            let v_rst: f32 = line.matches('V').count() as f32; // neither T nor U
            let v_tot: f32 = data.matches('V').count() as f32;
            let v_seq: f32 = v_tot - v_rst;

            let n_rst: f32 = line.matches('N').count() as f32; // any nucleotide
            let n_tot: f32 = data.matches('N').count() as f32;
            let n_seq: f32 = n_tot - n_rst;

            let z_rst: f32 = line.matches('Z').count() as f32; // zero
            let z_tot: f32 = data.matches('Z').count() as f32;
            let z_seq: f32 = z_tot - z_rst;

            let gap_rst: f32 = line.matches('-').count() as f32; // gap
            let gap_tot: f32 = data.matches('-').count() as f32;
            let gap_seq: f32 = gap_tot - gap_rst;

            // CALCULATIONS

            let sum_nuc: f32 = ade_seq + cyt_seq + gua_seq + thy_seq + ura_seq;
            let sum_amb: f32 = w_seq + s_seq + m_seq + k_seq + r_seq + y_seq + b_seq + d_seq + h_seq + v_seq + n_seq;
            let sum_seq: f32 = sum_nuc + sum_amb;

            let at_sum: f32 = ade_seq + thy_seq;
            let au_sum: f32 = ade_seq + ura_seq;
            let cg_sum: f32 = cyt_seq + gua_seq;

            let at_per: f32 = at_sum / sum_seq * 100.0;
            let au_per: f32 = au_sum / sum_seq * 100.0;
            let cg_per: f32 = cg_sum / sum_seq * 100.0;

            let ade_per: f32 = ade_seq / sum_seq * 100.0;
            let cyt_per: f32 = cyt_seq / sum_seq * 100.0;
            let gua_per: f32 = gua_seq / sum_seq * 100.0;
            let thy_per: f32 = thy_seq / sum_seq * 100.0;
            let ura_per: f32 = ura_seq / sum_seq * 100.0;

            let w_per: f32 = w_seq / sum_seq * 100.0;
            let s_per: f32 = s_seq / sum_seq * 100.0;
            let m_per: f32 = m_seq / sum_seq * 100.0;
            let k_per: f32 = k_seq / sum_seq * 100.0;
            let r_per: f32 = r_seq / sum_seq * 100.0;
            let y_per: f32 = y_seq / sum_seq * 100.0;
            let b_per: f32 = b_seq / sum_seq * 100.0;
            let d_per: f32 = d_seq / sum_seq * 100.0;
            let h_per: f32 = h_seq / sum_seq * 100.0;
            let v_per: f32 = v_seq / sum_seq * 100.0;
            let n_per: f32 = n_seq / sum_seq * 100.0;

            // DISPLAY

            print!("{}", red.to_owned());
            println!("{}", line);
            println!("{}", reset.to_owned());
            if sum_seq > 0.0 {
                print!("Sequence length: {}", yellow.to_owned());
                print!("{}", sum_seq);
                print!("{}", reset);
                println!(" nt");
                println!();
            } else {
                panic!("{}", red.to_owned() + "The sequence is empty!" + reset);
            }

            if thy_seq > 0.0 {
                print!("Total {}", cyan.to_owned() + "A-T" + reset + ":  " + yellow);
                print!("{}", at_sum);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", at_per);
                print!("{}", reset);
                println!(" %");
            }
            if ura_seq > 0.0 {
                print!("Total {}", cyan.to_owned() + "A-U" + reset + ":  " + yellow);
                print!("{}", au_sum);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", au_per);
                print!("{}", reset);
                println!(" %");
            }
            if cg_sum > 0.0 {
                print!("Total {}", cyan.to_owned() + "C-G" + reset + ":  " + yellow);
                print!("{}", cg_sum);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", cg_per);
                print!("{}", reset);
                println!(" %");
            }

            if sum_nuc > 0.0 {
                println!();
                println!("Nucleobases:");
                println!();
            }
            if ade_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Adenine" + reset + ":    " + yellow);
                print!("{}", ade_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", ade_per);
                print!("{}", reset);
                println!(" %");
            }
            if cyt_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Cytosine" + reset + ":   " + yellow);
                print!("{}", cyt_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", cyt_per);
                print!("{}", reset);
                println!(" %");
            }
            if gua_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Guanine" + reset + ":    " + yellow);
                print!("{}", gua_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", gua_per);
                print!("{}", reset);
                println!(" %");
            }
            if thy_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Thymine" + reset + ":    " + yellow);
                print!("{}", thy_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", thy_per);
                print!("{}", reset);
                println!(" %");
            }
            if ura_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Uracil" + reset + ":     " + yellow);
                print!("{}", ura_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", ura_per);
                print!("{}", reset);
                println!(" %");
            }

            if sum_amb > 0.0 {
                if sum_nuc > 0.0 {
                    println!();
                }
                println!("Ambiguous:");
                println!();
            }
            if w_seq > 0.0 {
                print!("{}", cyan.to_owned() + "weak" + reset + ":       " + yellow);
                print!("{}", w_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", w_per);
                print!("{}", reset);
                println!(" %");
            }
            if s_seq > 0.0 {
                print!("{}", cyan.to_owned() + "strong" + reset + ":     " + yellow);
                print!("{}", s_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", s_per);
                print!("{}", reset);
                println!(" %");
            }
            if m_seq > 0.0 {
                print!("{}", cyan.to_owned() + "amino" + reset + ":      " + yellow);
                print!("{}", m_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", m_per);
                print!("{}", reset);
                println!(" %");
            }
            if k_seq > 0.0 {
                print!("{}", cyan.to_owned() + "keto" + reset + ":       " + yellow);
                print!("{}", k_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", k_per);
                print!("{}", reset);
                println!(" %");
            }
            if r_seq > 0.0 {
                print!("{}", cyan.to_owned() + "purine" + reset + ":     " + yellow);
                print!("{}", r_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", r_per);
                print!("{}", reset);
                println!(" %");
            }
            if y_seq > 0.0 {
                print!("{}", cyan.to_owned() + "pyrimidine" + reset + ": " + yellow);
                print!("{}", y_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", y_per);
                print!("{}", reset);
                println!(" %");
            }
            if b_seq > 0.0 {
                print!("{}", cyan.to_owned() + "not A" + reset + ":      " + yellow);
                print!("{}", b_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", b_per);
                print!("{}", reset);
                println!(" %");
            }
            if d_seq > 0.0 {
                print!("{}", cyan.to_owned() + "not C" + reset + ":      " + yellow);
                print!("{}", d_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", d_per);
                print!("{}", reset);
                println!(" %");
            }
            if h_seq > 0.0 {
                print!("{}", cyan.to_owned() + "not G" + reset + ":      " + yellow);
                print!("{}", h_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", h_per);
                print!("{}", reset);
                println!(" %");
            }
            if v_seq > 0.0 {
                if ura_seq > 0.0 {
                    print!("{}", cyan.to_owned() + "not U" + reset + ":      " + yellow);
                } else {
                    print!("{}", cyan.to_owned() + "not T" + reset + ":      " + yellow);
                }
                print!("{}", v_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", v_per);
                print!("{}", reset);
                println!(" %");
            }
            if n_seq > 0.0 {
                print!("{}", cyan.to_owned() + "any" + reset + ":        " + yellow);
                print!("{}", n_seq);
                print!("{}", reset);
                print!(" nt, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", n_per);
                print!("{}", reset);
                println!(" %");
            }
            if z_seq > 0.0 {
                print!("{}", cyan.to_owned() + "zero" + reset + ":       " + yellow);
                print!("{}", z_seq);
                print!("{}", reset);
                println!(" nt");
            }
            if gap_seq > 0.0 {
                print!("{}", cyan.to_owned() + "gap" + reset + ":        " + yellow);
                print!("{}", gap_seq);
                print!("{}", reset);
                println!(" nt");
            }
            return;
        }
    }
    panic!("{}", red.to_owned() + "No sequence or incorrect file format!" + reset);
}
