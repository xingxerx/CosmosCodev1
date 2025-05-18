import re
from qiskit import QuantumCircuit, execute
from qiskit_aer import Aer
from qiskit.qasm2 import dumps

# ====================================================
#            Binary Translator Functions
# ====================================================

def convert_parameter_to_binary(param):
    """
    Convert a numerical parameter to a 16-bit binary string.
    For this demonstration, multiply the parameter by 10,000
    to preserve four decimal places.
    """
    try:
        value = float(param)
        scaled = int(value * 10000)
        return format(scaled, '016b')
    except ValueError:
        # Fall back to ASCII binary conversion if needed.
        return "".join(format(ord(c), '08b') for c in param)

def convert_qasm_to_binary(qasm_str):
    """
    Convert QASM output into a simplified binary representation.
    
    - For a parameterized 'u' gate (which represents a u3 gate),
      use "0100" as the code followed by each scaled parameter in binary.
    - Use a mapping for other known gates.
    - For unknown tokens, convert each character to its 8-bit ASCII binary representation.
    """
    binary_code = ""
    
    # Pattern to match the parameterized gate line.
    # Note: Qiskit prints the u3 gate as "u(...)" in QASM.
    u3_pattern = re.compile(r"^\s*u\(([^)]+)\)\s+(\w+\[\d+\]);")
    
    for line in qasm_str.splitlines():
        # Skip headers and register declaration lines.
        if (line.startswith("OPENQASM") or line.startswith("include") or 
            line.startswith("qreg") or line.startswith("creg")):
            continue

        # Check for a u gate (parameterized gate) line.
        u3_match = u3_pattern.match(line)
        if u3_match:
            binary_code += "0100 "  # Our fixed binary code for the 'u' gate.
            params_str = u3_match.group(1)
            # Split and convert parameters.
            params = params_str.split(',')
            for p in params:
                binary_code += convert_parameter_to_binary(p.strip()) + " "
            continue

        # For other gates, extract the first word and map it.
        match = re.match(r"^\s*(\w+)", line)
        if match:
            gate = match.group(1)
            # Extend the mapping as you add more gates.
            gate_mapping = {
                "h": "0001",      # Hadamard gate
                "cx": "0010",     # CNOT gate
                "measure": "0011",
                "x": "0101",      # Pauli-X gate
                "z": "0110",      # Pauli-Z gate
            }
            if gate in gate_mapping:
                binary_code += gate_mapping[gate] + " "
            else:
                # Convert unknown gate names to their ASCII binary representation.
                for char in gate:
                    binary_code += format(ord(char), '08b') + " "
                binary_code += " "
    return binary_code.strip()

# ====================================================
#      Superdense Coding Circuit Construction
# ====================================================

def superdense_coding_circuit(message='00'):
    """
    Constructs the superdense coding circuit.
    
    Parameters:
      message (str): A two-bit message ('00', '01', '10', or '11') that Alice wishes to send.
      
    The procedure:
      1. Create a Bell pair: entangle qubits using H and CX gates.
      2. Alice encodes her two-bit message on qubit 0.
         - '00': Do nothing.
         - '01': Apply X.
         - '10': Apply Z.
         - '11': Apply X then Z.
      3. Bob decodes by applying CX and H, then measuring both qubits.
    """
    qc = QuantumCircuit(2, 2)
    
    # Create the Bell pair.
    qc.h(0)
    qc.cx(0, 1)
    
    # Encoding: Alice applies specific Pauli operations on qubit 0.
    if message == '01':
        qc.x(0)
    elif message == '10':
        qc.z(0)
    elif message == '11':
        qc.x(0)
        qc.z(0)
    # '00' means no operation.
    
    # Decoding on Bob's side.
    qc.cx(0, 1)
    qc.h(0)
    
    # Measure both qubits to decode the message.
    qc.measure(0, 0)
    qc.measure(1, 1)
    
    return qc

# ====================================================
#                   Main Function
# ====================================================

def main():
    # Choose a message to encode (e.g., '11').
    message = '11'  # You can change this to '00', '01', or '10' as needed.
    qc = superdense_coding_circuit(message)
    
    # Export the QASM representation.
    qasm_str = dumps(qc)
    print("QASM Output:")
    print(qasm_str)
    
    # Convert the QASM to a binary translation.
    binary_representation = convert_qasm_to_binary(qasm_str)
    print("\nBinary Translation:")
    print(binary_representation)
    
    # Simulate the circuit to verify the decoded message.
    backend = Aer.get_backend('qasm_simulator')
    job = execute(qc, backend, shots=1024)
    counts = job.result().get_counts()
    print("\nSimulation Results (Decoded Message Distribution):")
    print(counts)

if __name__ == '__main__':
    main()