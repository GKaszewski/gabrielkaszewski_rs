const deleteForms = document.querySelectorAll('.delete-form');

const deleteData = async (form) => {
  const fileIdInput = form.querySelector('.file-id');
  const fileId = fileIdInput.value;
  if (!fileId) {
    console.warn('No file selected');
    return;
  }

  try {
    const response = await fetch(`/api/data/id/${fileId}`, {
      method: 'DELETE',
    });

    if (response.ok) {
      alert('Data deleted successfully');
      window.location.reload();
      fileIdInput.value = '';
    } else {
      console.error(
        'Failed to delete data ',
        response.status,
        response.statusText
      );
    }
  } catch (error) {
    console.error('Error deleting data ', error);
  }
};

deleteForms.forEach((form) => {
  const deleteButton = form.querySelector('.delete-button');
  deleteButton.addEventListener('click', (event) => {
    event.preventDefault();
    deleteData(form);
  });
});
