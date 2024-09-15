# Extracts all the .zip files in the dist/ directory and puts the singular file in the extracted folder in the extracted/ directory
import zipfile
import os

# Gets all the .zip files in the dist/ directory
zip_files = [f for f in os.listdir('dist/') if f.endswith('.zip')]

# Extracts all the .zip files in the dist/ directory
for zip_file in zip_files:
    with zipfile.ZipFile('dist/' + zip_file, 'r') as zip_ref:
        zip_ref.extractall('extracted/')
        print('Extracted ' + zip_file + ' to extracted/')