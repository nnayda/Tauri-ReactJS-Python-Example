import uvicorn
from main import app

if __name__ == "__main__":
    config = uvicorn.Config(app, port=51236, log_level="info")
    server = uvicorn.Server(config)
    server.run()
