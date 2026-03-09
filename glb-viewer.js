const viewer = document.querySelector('#viewer');
const progressBar = document.querySelector('#progress-bar');
const modelNameDisplay = document.querySelector('#model-name');
const infoPanel = document.querySelector('#info');
const modelStats = document.querySelector('#model-stats');
const errorMessage = document.querySelector('#error');

// Extract model from URL parameter
const urlParams = new URLSearchParams(window.location.search);
const modelPath = urlParams.get('model');

if (modelPath) {
    // Ensure the path is root-relative if it's not absolute
    let finalPath = modelPath;
    if (!modelPath.startsWith('/') && !modelPath.startsWith('http') && !modelPath.startsWith('./') && !modelPath.startsWith('../')) {
        finalPath = '/' + modelPath;
    }
    viewer.src = finalPath;
    modelNameDisplay.textContent = modelPath.split('/').pop();
} else {
    errorMessage.style.display = 'block';
    modelNameDisplay.textContent = 'No model selected';
}

// Handle loading events
viewer.addEventListener('progress', (event) => {
    const progress = event.detail.totalProgress * 100;
    progressBar.style.width = `${progress}%`;
    if (progress === 100) {
        setTimeout(() => {
            progressBar.style.opacity = '0';
        }, 500);
    }
});

viewer.addEventListener('load', () => {
    infoPanel.style.display = 'block';

    // Get some basic info if possible
    // Note: model-viewer doesn't expose many internal stats easily without delving into Three.js
    // but we can show the source path at least.
    modelStats.innerHTML = `
        <div class="property">
            <span>Source</span>
            <span>${modelPath}</span>
        </div>
        <div class="property">
            <span>Format</span>
            <span>GLB (glTF)</span>
        </div>
    `;
});

viewer.addEventListener('error', (e) => {
    console.error('Error loading model:', e);
    errorMessage.style.display = 'block';
});
