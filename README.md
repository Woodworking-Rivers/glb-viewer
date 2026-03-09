# GLB Viewer

A 3D model viewer for GLB and glTF files, powered by Google's `<model-viewer>`.

## Usage

### 1. Run the Server

```bash
bazel run //:server /path/to/assets
```

Provide the directory containing your `.glb` files as an argument. By default, it serves the current directory.

### 2. View Models

Open your browser and navigate to:

```
http://localhost:8000/glb-viewer.html?model=path/to/model.glb
```

The `model` parameter should be the relative path to your file from the served directory.

## Controls

- **Rotate**: Left Mouse Button / One-finger Touch
- **Zoom**: Mouse Wheel / Two-finger Pinch
- **Pan**: Right Mouse Button / Two-finger Drag

## Internal Tech

- **3D Engine**: [Google `<model-viewer>`](https://modelviewer.dev/)
- **Frontend**: HTML5, CSS, JS
- **Server**: Python (Bazel managed)
