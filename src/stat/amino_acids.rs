// NUCLEOB VERSION 1.0.0 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE AMINO_ACIDS

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::similar_names, clippy::too_many_lines)]

// FUNCTION

pub fn a_count(data: &str) {
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

    // AMINO ACIDS

    for line in data.split('\n') {
        if line.starts_with('>') {
            let ala_rst: f32 = line.matches('A').count() as f32; // Alanine
            let ala_tot: f32 = data.matches('A').count() as f32;
            let ala_seq: f32 = ala_tot - ala_rst;

            let arg_rst: f32 = line.matches('R').count() as f32; // Arginine
            let arg_tot: f32 = data.matches('R').count() as f32;
            let arg_seq: f32 = arg_tot - arg_rst;

            let asn_rst: f32 = line.matches('N').count() as f32; // Asparagine
            let asn_tot: f32 = data.matches('N').count() as f32;
            let asn_seq: f32 = asn_tot - asn_rst;

            let asp_rst: f32 = line.matches('D').count() as f32; // Aspartate
            let asp_tot: f32 = data.matches('D').count() as f32;
            let asp_seq: f32 = asp_tot - asp_rst;

            let cys_rst: f32 = line.matches('C').count() as f32; // Cysteine
            let cys_tot: f32 = data.matches('C').count() as f32;
            let cys_seq: f32 = cys_tot - cys_rst;

            let gln_rst: f32 = line.matches('Q').count() as f32; // Glutamine
            let gln_tot: f32 = data.matches('Q').count() as f32;
            let gln_seq: f32 = gln_tot - gln_rst;

            let glu_rst: f32 = line.matches('E').count() as f32; // Glutamate
            let glu_tot: f32 = data.matches('E').count() as f32;
            let glu_seq: f32 = glu_tot - glu_rst;

            let gly_rst: f32 = line.matches('G').count() as f32; // Glycine
            let gly_tot: f32 = data.matches('G').count() as f32;
            let gly_seq: f32 = gly_tot - gly_rst;

            let his_rst: f32 = line.matches('H').count() as f32; // Histidine
            let his_tot: f32 = data.matches('H').count() as f32;
            let his_seq: f32 = his_tot - his_rst;

            let ile_rst: f32 = line.matches('I').count() as f32; // Isoleucine
            let ile_tot: f32 = data.matches('I').count() as f32;
            let ile_seq: f32 = ile_tot - ile_rst;

            let leu_rst: f32 = line.matches('L').count() as f32; // Leucine
            let leu_tot: f32 = data.matches('L').count() as f32;
            let leu_seq: f32 = leu_tot - leu_rst;

            let lys_rst: f32 = line.matches('K').count() as f32; // Lysine
            let lys_tot: f32 = data.matches('K').count() as f32;
            let lys_seq: f32 = lys_tot - lys_rst;

            let met_rst: f32 = line.matches('M').count() as f32; // Methionine
            let met_tot: f32 = data.matches('M').count() as f32;
            let met_seq: f32 = met_tot - met_rst;

            let phe_rst: f32 = line.matches('F').count() as f32; // Phenylalanine
            let phe_tot: f32 = data.matches('F').count() as f32;
            let phe_seq: f32 = phe_tot - phe_rst;

            let pro_rst: f32 = line.matches('P').count() as f32; // Proline
            let pro_tot: f32 = data.matches('P').count() as f32;
            let pro_seq: f32 = pro_tot - pro_rst;

            let ser_rst: f32 = line.matches('S').count() as f32; // Serine
            let ser_tot: f32 = data.matches('S').count() as f32;
            let ser_seq: f32 = ser_tot - ser_rst;

            let thr_rst: f32 = line.matches('T').count() as f32; // Threonine
            let thr_tot: f32 = data.matches('T').count() as f32;
            let thr_seq: f32 = thr_tot - thr_rst;

            let trp_rst: f32 = line.matches('W').count() as f32; // Tryptophan
            let trp_tot: f32 = data.matches('W').count() as f32;
            let trp_seq: f32 = trp_tot - trp_rst;

            let tyr_rst: f32 = line.matches('Y').count() as f32; // Tyrosine
            let tyr_tot: f32 = data.matches('Y').count() as f32;
            let tyr_seq: f32 = tyr_tot - tyr_rst;

            let val_rst: f32 = line.matches('V').count() as f32; // Valine
            let val_tot: f32 = data.matches('V').count() as f32;
            let val_seq: f32 = val_tot - val_rst;

            let sec_rst: f32 = line.matches('U').count() as f32; // Selenocysteine
            let sec_tot: f32 = data.matches('U').count() as f32;
            let sec_seq: f32 = sec_tot - sec_rst;

            let pyl_rst: f32 = line.matches('O').count() as f32; // Pyrrolysine
            let pyl_tot: f32 = data.matches('O').count() as f32;
            let pyl_seq: f32 = pyl_tot - pyl_rst;

            // AMBIGUOUS

            let xaa_rst: f32 = line.matches('X').count() as f32; // Any
            let xaa_tot: f32 = data.matches('X').count() as f32;
            let xaa_seq: f32 = xaa_tot - xaa_rst;

            let asx_rst: f32 = line.matches('B').count() as f32; // Asparagine or aspartate
            let asx_tot: f32 = data.matches('B').count() as f32;
            let asx_seq: f32 = asx_tot - asx_rst;

            let glx_rst: f32 = line.matches('Z').count() as f32; // Glutamine or glutamate
            let glx_tot: f32 = data.matches('Z').count() as f32;
            let glx_seq: f32 = glx_tot - glx_rst;

            let xle_rst: f32 = line.matches('J').count() as f32; // Leucine or isoleucine
            let xle_tot: f32 = data.matches('J').count() as f32;
            let xle_seq: f32 = xle_tot - xle_rst;

            let hydrophobic_rst: f32 = line.matches('Φ').count() as f32; // Hydrophobic
            let hydrophobic_tot: f32 = data.matches('Φ').count() as f32;
            let hydrophobic_seq: f32 = hydrophobic_tot - hydrophobic_rst;

            let aromatic_rst: f32 = line.matches('Ω').count() as f32; // Aromatic
            let aromatic_tot: f32 = data.matches('Ω').count() as f32;
            let aromatic_seq: f32 = aromatic_tot - aromatic_rst;

            let aliphatic_rst: f32 = line.matches('Ψ').count() as f32; // Aliphatic
            let aliphatic_tot: f32 = data.matches('Ψ').count() as f32;
            let aliphatic_seq: f32 = aliphatic_tot - aliphatic_rst;

            let small_rst: f32 = line.matches('π').count() as f32; // Small
            let small_tot: f32 = data.matches('π').count() as f32;
            let small_seq: f32 = small_tot - small_rst;

            let hydrophilic_rst: f32 = line.matches('ζ').count() as f32; // Hydrophilic
            let hydrophilic_tot: f32 = data.matches('ζ').count() as f32;
            let hydrophilic_seq: f32 = hydrophilic_tot - hydrophilic_rst;

            let positively_charged_rst: f32 = line.matches('+').count() as f32; // Positively-charged
            let positively_charged_tot: f32 = data.matches('+').count() as f32;
            let positively_charged_seq: f32 = positively_charged_tot - positively_charged_rst;

            let negatively_charged_rst: f32 = line.matches('−').count() as f32; // Negatively-charged
            let negatively_charged_tot: f32 = data.matches('−').count() as f32;
            let negatively_charged_seq: f32 = negatively_charged_tot - negatively_charged_rst;

            // CALCULATIONS

            let sum_ami: f32 = ala_seq + arg_seq + asn_seq + asp_seq + cys_seq + gln_seq + glu_seq + gly_seq + his_seq + ile_seq + leu_seq + lys_seq + met_seq + phe_seq + pro_seq + ser_seq + thr_seq + trp_seq + tyr_seq + val_seq + sec_seq + pyl_seq;
            let sum_amb: f32 = xaa_seq + asx_seq + glx_seq + xle_seq + hydrophobic_seq + aromatic_seq + aliphatic_seq + small_seq + hydrophilic_seq + positively_charged_seq + negatively_charged_seq;
            let sum_seq: f32 = sum_ami + sum_amb;

            let ala_per: f32 = ala_seq / sum_seq * 100.0;
            let arg_per: f32 = arg_seq / sum_seq * 100.0;
            let asn_per: f32 = asn_seq / sum_seq * 100.0;
            let asp_per: f32 = asp_seq / sum_seq * 100.0;
            let cys_per: f32 = cys_seq / sum_seq * 100.0;
            let gln_per: f32 = gln_seq / sum_seq * 100.0;
            let glu_per: f32 = glu_seq / sum_seq * 100.0;
            let gly_per: f32 = gly_seq / sum_seq * 100.0;
            let his_per: f32 = his_seq / sum_seq * 100.0;
            let ile_per: f32 = ile_seq / sum_seq * 100.0;
            let leu_per: f32 = leu_seq / sum_seq * 100.0;
            let lys_per: f32 = lys_seq / sum_seq * 100.0;
            let met_per: f32 = met_seq / sum_seq * 100.0;
            let phe_per: f32 = phe_seq / sum_seq * 100.0;
            let pro_per: f32 = pro_seq / sum_seq * 100.0;
            let ser_per: f32 = ser_seq / sum_seq * 100.0;
            let thr_per: f32 = thr_seq / sum_seq * 100.0;
            let trp_per: f32 = trp_seq / sum_seq * 100.0;
            let tyr_per: f32 = tyr_seq / sum_seq * 100.0;
            let val_per: f32 = val_seq / sum_seq * 100.0;
            let sec_per: f32 = sec_seq / sum_seq * 100.0;
            let pyl_per: f32 = pyl_seq / sum_seq * 100.0;

            let xaa_per: f32 = xaa_seq / sum_seq * 100.0;
            let asx_per: f32 = asx_seq / sum_seq * 100.0;
            let glx_per: f32 = glx_seq / sum_seq * 100.0;
            let xle_per: f32 = xle_seq / sum_seq * 100.0;
            let hydrophobic_per: f32 = hydrophobic_seq / sum_seq * 100.0;
            let aromatic_per: f32 = aromatic_seq / sum_seq * 100.0;
            let aliphatic_per: f32 = aliphatic_seq / sum_seq * 100.0;
            let small_per: f32 = small_seq / sum_seq * 100.0;
            let hydrophilic_per: f32 = hydrophilic_seq / sum_seq * 100.0;
            let positively_charged_per: f32 = positively_charged_seq / sum_seq * 100.0;
            let negatively_charged_per: f32 = negatively_charged_seq / sum_seq * 100.0;

            // DISPLAY

            print!("{}", red.to_owned());
            println!("{}", line);
            println!("{}", reset.to_owned());
            if sum_seq > 0.0 {
                print!("Sequence length: {}", yellow.to_owned());
                print!("{}", sum_seq);
                print!("{}", reset);
                println!(" aa");
                println!();
            } else {
                panic!("{}", red.to_owned() + "The sequence is empty!" + reset);
            }

            if sum_ami > 0.0 {
                println!("Amino acids:");
                println!();
            }
            if ala_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Alanine" + reset + ":        " + yellow);
                print!("{}", ala_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", ala_per);
                print!("{}", reset);
                println!(" %");
            }
            if arg_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Arginine" + reset + ":       " + yellow);
                print!("{}", arg_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", arg_per);
                print!("{}", reset);
                println!(" %");
            }
            if asn_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Asparagine" + reset + ":     " + yellow);
                print!("{}", asn_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", asn_per);
                print!("{}", reset);
                println!(" %");
            }
            if asp_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Aspartate" + reset + ":      " + yellow);
                print!("{}", asp_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", asp_per);
                print!("{}", reset);
                println!(" %");
            }
            if cys_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Cysteine" + reset + ":       " + yellow);
                print!("{}", cys_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", cys_per);
                print!("{}", reset);
                println!(" %");
            }
            if gln_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Glutamine" + reset + ":      " + yellow);
                print!("{}", gln_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", gln_per);
                print!("{}", reset);
                println!(" %");
            }
            if glu_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Glutamate" + reset + ":      " + yellow);
                print!("{}", glu_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", glu_per);
                print!("{}", reset);
                println!(" %");
            }
            if gly_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Glycine" + reset + ":        " + yellow);
                print!("{}", gly_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", gly_per);
                print!("{}", reset);
                println!(" %");
            }
            if his_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Histidine" + reset + ":      " + yellow);
                print!("{}", his_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", his_per);
                print!("{}", reset);
                println!(" %");
            }
            if ile_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Isoleucine" + reset + ":     " + yellow);
                print!("{}", ile_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", ile_per);
                print!("{}", reset);
                println!(" %");
            }
            if leu_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Leucine" + reset + ":        " + yellow);
                print!("{}", leu_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", leu_per);
                print!("{}", reset);
                println!(" %");
            }
            if lys_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Lysine" + reset + ":         " + yellow);
                print!("{}", lys_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", lys_per);
                print!("{}", reset);
                println!(" %");
            }
            if met_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Methionine" + reset + ":     " + yellow);
                print!("{}", met_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", met_per);
                print!("{}", reset);
                println!(" %");
            }
            if phe_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Phenylalanine" + reset + ":  " + yellow);
                print!("{}", phe_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", phe_per);
                print!("{}", reset);
                println!(" %");
            }
            if pro_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Proline" + reset + ":        " + yellow);
                print!("{}", pro_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", pro_per);
                print!("{}", reset);
                println!(" %");
            }
            if ser_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Serine" + reset + ":         " + yellow);
                print!("{}", ser_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", ser_per);
                print!("{}", reset);
                println!(" %");
            }
            if thr_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Threonine" + reset + ":      " + yellow);
                print!("{}", thr_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", thr_per);
                print!("{}", reset);
                println!(" %");
            }
            if trp_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Tryptophan" + reset + ":     " + yellow);
                print!("{}", trp_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", trp_per);
                print!("{}", reset);
                println!(" %");
            }
            if tyr_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Tyrosine" + reset + ":       " + yellow);
                print!("{}", tyr_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", tyr_per);
                print!("{}", reset);
                println!(" %");
            }
            if val_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Valine" + reset + ":         " + yellow);
                print!("{}", val_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", val_per);
                print!("{}", reset);
                println!(" %");
            }
            if sec_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Selenocysteine" + reset + ": " + yellow);
                print!("{}", sec_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", sec_per);
                print!("{}", reset);
                println!(" %");
            }
            if pyl_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Pyrrolysine" + reset + ":    " + yellow);
                print!("{}", pyl_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", pyl_per);
                print!("{}", reset);
                println!(" %");
            }

            if sum_amb > 0.0 {
                if sum_ami > 0.0 {
                    println!();
                }
                println!("Ambiguous:");
                println!();
            }
            if xaa_seq > 0.0 {
                print!("{}", cyan.to_owned() + "any" + reset + ":            " + yellow);
                print!("{}", xaa_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", xaa_per);
                print!("{}", reset);
                println!(" %");
            }
            if asx_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Asn or Asp" + reset + ":     " + yellow);
                print!("{}", asx_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", asx_per);
                print!("{}", reset);
                println!(" %");
            }
            if glx_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Gln or Glu" + reset + ":     " + yellow);
                print!("{}", glx_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", glx_per);
                print!("{}", reset);
                println!(" %");
            }
            if xle_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Leu or Ile" + reset + ":     " + yellow);
                print!("{}", xle_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", xle_per);
                print!("{}", reset);
                println!(" %");
            }
            if hydrophobic_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Hydrophobic" + reset + ":    " + yellow);
                print!("{}", hydrophobic_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", hydrophobic_per);
                print!("{}", reset);
                println!(" %");
            }
            if aromatic_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Aromatic" + reset + ":       " + yellow);
                print!("{}", aromatic_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", aromatic_per);
                print!("{}", reset);
                println!(" %");
            }
            if aliphatic_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Aliphatic" + reset + ":      " + yellow);
                print!("{}", aliphatic_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", aliphatic_per);
                print!("{}", reset);
                println!(" %");
            }
            if small_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Small" + reset + ":          " + yellow);
                print!("{}", small_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", small_per);
                print!("{}", reset);
                println!(" %");
            }
            if hydrophilic_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Hydrophilic" + reset + ":    " + yellow);
                print!("{}", hydrophilic_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", hydrophilic_per);
                print!("{}", reset);
                println!(" %");
            }
            if positively_charged_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Pos.-charged" + reset + ":   " + yellow);
                print!("{}", positively_charged_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", positively_charged_per);
                print!("{}", reset);
                println!(" %");
            }
            if negatively_charged_seq > 0.0 {
                print!("{}", cyan.to_owned() + "Neg.-charged" + reset + ":   " + yellow);
                print!("{}", negatively_charged_seq);
                print!("{}", reset);
                print!(" aa, ");
                print!("{}", yellow.to_owned());
                print!("{:.3}", negatively_charged_per);
                print!("{}", reset);
                println!(" %");
            }
            return;
        }
    }
    panic!("{}", red.to_owned() + "No sequence or incorrect file format!" + reset);
}
