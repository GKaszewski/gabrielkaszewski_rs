const form = document.getElementById('data-upload');
const fileInput = document.getElementById('file-input');
const protectedInput = document.getElementById('protected-input');

const uploadData = async () => {
  if (!fileInput.files.length) {
    console.warn('No file selected');
    return;
  }

  const formData = new FormData();
  formData.append('file', fileInput.files[0]);
  formData.append('protected', protectedInput.checked ? 'true' : 'false');

  try {
    const response = await fetch('/api/data/upload', {
      method: 'POST',
      body: formData,
    });

    if (response.ok) {
      alert('Data uploaded successfully');
      form.reset();
    } else {
      console.error(
        'Failed to upload data ',
        response.status,
        response.statusText
      );
    }
  } catch (error) {
    console.error('Error uploading data ', error);
  }
};

form.addEventListener('submit', (event) => {
  event.preventDefault();
  uploadData();
});
