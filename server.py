import argparse
import http.server
import socketserver
import os
import sys
import signal
import urllib.parse

PORT = 8000

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, viewer_dir=None, data_dir=None, **kwargs):
        self.viewer_dir = viewer_dir
        self.data_dir = data_dir
        # Initialize with one of them, we'll override translate_path
        super().__init__(*args, directory=viewer_dir, **kwargs)

    def translate_path(self, path):
        # First, try to find the file in the viewer_dir (HTML, JS, CSS)
        # This allows the viewer to be served even if started elsewhere
        path_in_viewer = super().translate_path(path)
        
        # If the file exists in viewer_dir, return that path
        if os.path.exists(path_in_viewer):
            return path_in_viewer
            
        # If not found in viewer_dir, look in data_dir
        # We need to manually translate the path for data_dir
        # SimpleHTTPRequestHandler.translate_path uses self.directory, 
        # but we can't easily swap it mid-call safely without race conditions 
        # if it were multi-threaded (though TCPServer is not by default).
        
        # Calculate trailing path from the URL
        url_path = path.split('?', 1)[0].split('#', 1)[0]
        url_path = urllib.parse.unquote(url_path)
        
        # Clean up the path to prevent directory traversal
        parts = url_path.split('/')
        parts = filter(None, parts)
        
        path_in_data = self.data_dir
        for part in parts:
            if os.path.dirname(part) or part in (os.curdir, os.pardir):
                continue
            path_in_data = os.path.join(path_in_data, part)
            
        return path_in_data

def main():
    parser = argparse.ArgumentParser(description="Serve GLB files.")
    parser.add_argument("directory", nargs="?", default=".", help="Directory to serve assets from (default: current directory)")
    args = parser.parse_args()

    # The directory where this script is located (containing glb-viewer.html etc.)
    viewer_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Try to find the workspace root if running via Bazel
    if "BUILD_WORKSPACE_DIRECTORY" in os.environ:
        os.chdir(os.environ["BUILD_WORKSPACE_DIRECTORY"])
    
    data_dir = os.path.abspath(args.directory)
    if not os.path.isdir(data_dir):
        print(f"Error: Directory '{data_dir}' does not exist.")
        sys.exit(1)

    print(f"Viewer directory: {viewer_dir}")
    print(f"Data directory:   {data_dir}")
    print(f"Serving at http://localhost:{PORT}")
    print(f"To view a model: http://localhost:{PORT}/glb-viewer.html?model=path/to/model.glb")
    
    # Create a factory for the handler with both directories
    def handler_factory(*args, **kwargs):
        return Handler(*args, viewer_dir=viewer_dir, data_dir=data_dir, **kwargs)

    def signal_handler(sig, frame):
        print(f"\nReceived signal {sig}. Shutting down server...")
        sys.exit(0)

    signal.signal(signal.SIGTERM, signal_handler)
    signal.signal(signal.SIGINT, signal_handler)

    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), handler_factory) as httpd:
        try:
            httpd.serve_forever()
        except (KeyboardInterrupt, SystemExit):
            httpd.server_close()

if __name__ == "__main__":
    main()
