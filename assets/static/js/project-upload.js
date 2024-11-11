document
  .getElementById('project-upload')
  .addEventListener('submit', function (event) {
    const fileInput = document.getElementById('files');

    if (fileInput.files.length === 0) {
      fileInput.removeAttribute('name');
    }
  });
