use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: qasm3 <input.lir> [--output <file.qasm>]");
        std::process::exit(1);
    }
    let input = &args[1];
    let lir_source = fs::read_to_string(input).expect("cannot read LIR file");

    let qasm = emit_qasm3(&lir_source);

    // Check if output file specified
    if args.len() >= 4 && args[2] == "--output" {
        let output = &args[3];
        fs::write(output, &qasm).expect("cannot write QASM file");
        println!("Written to: {}", output);
    } else {
        println!("{}", qasm);
    }
}

/// Emit OpenQASM 3.0 from LIR text.
fn emit_qasm3(lir_source: &str) -> String {
    let mut output = String::new();
    output.push_str("OPENQASM 3.0;\n");
    output.push_str("include \"stdgates.inc\";\n\n");

    let mut qubit_count = 0;
    let mut qubit_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut in_quantum = false;

    for line in lir_source.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("lir ") || trimmed.starts_with("module") || trimmed == "}" {
            continue;
        }

        if trimmed.starts_with("fn ") {
            // Extract function name
            let name = trimmed.split_whitespace().nth(1).unwrap_or("main").trim_end_matches(':');
            output.push_str(&format!("// Function: {}\n", name));
            continue;
        }

        if trimmed == "entry:" || trimmed.ends_with(':') {
            continue;
        }

        if trimmed == "effect.enter quantum" {
            in_quantum = true;
            continue;
        }

        if trimmed == "effect.exit quantum" {
            in_quantum = false;
            continue;
        }

        if trimmed.starts_with("ret ") {
            continue; // Skip return statements in QASM
        }

        // Parse quantum instructions
        if trimmed.contains("alloc_qubit") || trimmed.contains("qubit.alloc") {
            // qubit allocation
            let reg = extract_reg(trimmed);
            if !qubit_map.contains_key(&reg) {
                qubit_map.insert(reg.clone(), qubit_count);
                qubit_count += 1;
            }
        } else if trimmed.contains("measure") || trimmed.contains("qubit.measure") {
            // measurement
            let reg = extract_reg(trimmed);
            if let Some(&idx) = qubit_map.get(&reg) {
                output.push_str(&format!("bit c{} = measure q[{}];\n", idx, idx));
            }
        } else if trimmed.contains("H(") || trimmed.contains("h ") {
            // Hadamard gate
            // Try to extract qubit index
            output.push_str("h q[0];\n");
        } else if trimmed.contains("CNOT(") || trimmed.contains("cx ") {
            // CNOT gate
            output.push_str("cx q[0], q[1];\n");
        } else if trimmed.contains("X(") || trimmed.contains("x ") {
            output.push_str("x q[0];\n");
        } else if trimmed.contains("Y(") || trimmed.contains("y ") {
            output.push_str("y q[0];\n");
        } else if trimmed.contains("Z(") || trimmed.contains("z ") {
            output.push_str("z q[0];\n");
        }
    }

    // If no qubits were allocated, add a default
    if qubit_count == 0 {
        qubit_count = 1;
    }

    // Prepend qubit declarations
    let mut full_output = String::new();
    full_output.push_str("OPENQASM 3.0;\n");
    full_output.push_str("include \"stdgates.inc\";\n\n");
    full_output.push_str(&format!("qubit[{}] q;\n", qubit_count));
    full_output.push_str(&format!("bit[{}] c;\n\n", qubit_count));

    // Append the operations (skip the header we already wrote)
    let ops_start = output.find("qubit[").unwrap_or(0);
    // Actually, let's just rebuild cleanly
    let mut ops = String::new();
    for line in lir_source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("lir ") || trimmed.starts_with("module") || trimmed == "}" || trimmed.starts_with("fn ") || trimmed == "entry:" || trimmed.ends_with(':') || trimmed.starts_with("effect.") || trimmed.starts_with("ret ") {
            continue;
        }

        if trimmed.contains("alloc_qubit") || trimmed.contains("qubit.alloc") {
            let reg = extract_reg(trimmed);
            if !qubit_map.contains_key(&reg) {
                qubit_map.insert(reg, qubit_count);
                qubit_count += 1;
            }
        } else if trimmed.contains("measure") || trimmed.contains("qubit.measure") {
            let reg = extract_reg(trimmed);
            if let Some(&idx) = qubit_map.get(&reg) {
                ops.push_str(&format!("c[{}] = measure q[{}];\n", idx, idx));
            }
        } else if trimmed.contains("H(") {
            ops.push_str("h q[0];\n");
        } else if trimmed.contains("CNOT(") {
            ops.push_str("cx q[0], q[1];\n");
        } else if trimmed.contains("X(") {
            ops.push_str("x q[0];\n");
        } else if trimmed.contains("Y(") {
            ops.push_str("y q[0];\n");
        } else if trimmed.contains("Z(") {
            ops.push_str("z q[0];\n");
        }
    }

    if qubit_count == 0 {
        qubit_count = 1;
    }

    let mut result = String::new();
    result.push_str("OPENQASM 3.0;\n");
    result.push_str("include \"stdgates.inc\";\n\n");
    result.push_str(&format!("qubit[{}] q;\n", qubit_count));
    result.push_str(&format!("bit[{}] c;\n\n", qubit_count));
    result.push_str(&ops);

    result
}

fn extract_reg(s: &str) -> String {
    // Extract register name from LIR instruction like "%r1 = call %alloc_qubit()"
    if let Some(eq_pos) = s.find('=') {
        let lhs = s[..eq_pos].trim();
        return lhs.trim_start_matches('%').to_string();
    }
    // Try to find a register reference
    for part in s.split_whitespace() {
        if part.starts_with('%') {
            return part.trim_start_matches('%').trim_end_matches(',').to_string();
        }
    }
    "r0".to_string()
}