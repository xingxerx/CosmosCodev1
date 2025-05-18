import json
import subprocess

def get_snmp_data():
    # Example SNMP command (modify for actual monitoring)
    result = subprocess.run(["snmpwalk", "-v2c", "-c", "public", "localhost", "1.3.6.1.2.1.2.2.1.10"], capture_output=True, text=True)
    return result.stdout.strip()

def get_wireguard_peers():
    result = subprocess.run(["wg", "show"], capture_output=True, text=True)
    return result.stdout.strip()

data = {
    "snmp_metrics": get_snmp_data(),
    "wireguard_status": get_wireguard_peers(),
}

with open("network_log.json", "w") as file:
    json.dump(data, file, indent=4)

print("Network data collected successfully!")