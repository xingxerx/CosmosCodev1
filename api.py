# Create api.py in F:\CosmosCode
from fastapi import FastAPI

app = FastAPI()

@app.get("/")
def root():
    return {"message": "CGX Omni WiFi API is running!"}

@app.get("/latency")
def get_latency():
    return {"latency": "50ms"}  # Replace with real latency logic later

@app.on_event("shutdown")
async def shutdown_event():
    print("Shutting down gracefully...")