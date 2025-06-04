document.addEventListener('DOMContentLoaded', () => {
    const arrayInput = document.getElementById('arrayInput');
    const processButton = document.getElementById('processButton');
    const tableBody = document.getElementById('tableBody');
    const errorMessage = document.getElementById('errorMessage');
    const noDataMessage = document.getElementById('noDataMessage');

    processButton.addEventListener('click', () => {
        errorMessage.textContent = '';
        tableBody.innerHTML = '';
        noDataMessage.style.display = 'none';

        const inputText = arrayInput.value.trim();
        if (!inputText) {
            errorMessage.textContent = 'Input cannot be empty.';
            noDataMessage.style.display = 'block';
            return;
        }

        let data;
        try {
            data = new Function(`return ${inputText}`)();
        } catch (error) {
            errorMessage.textContent = `Invalid array format: ${error.message}. Please ensure it's a valid JavaScript array of arrays.`;
            noDataMessage.style.display = 'block';
            return;
        }

        if (!Array.isArray(data)) {
            errorMessage.textContent = 'Input must be an array.';
            noDataMessage.style.display = 'block';
            return;
        }

        if (data.length === 0) {
            noDataMessage.textContent = 'The provided array is empty.';
            noDataMessage.style.display = 'block';
            return;
        }

        let allRowsValid = true;
        data.forEach((rowArray, rowIndex) => {
            if (!Array.isArray(rowArray)) {
                errorMessage.textContent = `Error: Element at index ${rowIndex} is not an array.`;
                allRowsValid = false;
                return;
            }
            if (rowArray.length !== 4) {
                errorMessage.textContent = `Error: Row ${rowIndex + 1} does not have exactly 4 elements. It has ${rowArray.length}.`;
                allRowsValid = false;
                return;
            }
        });

        if (!allRowsValid) {
            tableBody.innerHTML = '';
            noDataMessage.style.display = 'block';
            noDataMessage.textContent = 'Please correct the input format based on the error above.';
            return;
        }

        data.forEach(rowArray => {
            const tr = document.createElement('tr');
            rowArray.forEach(itemText => {
                const td = document.createElement('td');
                const textareaElement = document.createElement('textarea'); // CHANGED from input
                textareaElement.value = String(itemText); // Ensure it's a string
                textareaElement.readOnly = true;         // Make it a placeholder
                // textareaElement.rows = 3; // Optional: Set default rows, or rely on CSS min-height
                td.appendChild(textareaElement);
                tr.appendChild(td);
            });
            tableBody.appendChild(tr);
        });

        if (tableBody.children.length === 0 && !errorMessage.textContent) {
             noDataMessage.textContent = 'No data to display. The array might be empty or invalid after processing.';
             noDataMessage.style.display = 'block';
        }
    });

    if (tableBody.children.length === 0) {
        noDataMessage.style.display = 'block';
    }
});
