import numpy as np

latency_samples = np.array([30, 32, 40, 200, 35, 38, 250])  # Example values
threshold = np.mean(latency_samples) + 2 * np.std(latency_samples)
anomalies = [x for x in latency_samples if x > threshold]

if anomalies:
    print(f"🚨 Traffic Spikes Detected: {anomalies}")