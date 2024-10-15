# Extracts all the .zip files in the dist/ directory and puts the singular file in the extracted folder in the extracted/ directory
import zipfile
import os

in_dir = 'dist/'
out_dir = 'extracted/'

# Gets all the .zip files in the dist/ directory
zip_files = [f for f in os.listdir(in_dir) if f.endswith('.zip')]

# Extracts all the .zip files in the dist/ directory
for zip_file in zip_files:
    with zipfile.ZipFile(in_dir + zip_file) as zip_ref:
        zip_ref.extractall(out_dir)
        print(f'Extracted {zip_file} to {out_dir}')
