# Extracts all the .zip files in the dist/ directory and puts the singular file in each extracted folder in the extracted/ directory
import zipfile
import os
from agithub.GitHub import GitHub
from pathlib import Path
import requests
import shutil

def download_file(url, file):
    local_filename = file
    # NOTE the stream=True parameter below
    with requests.get(url, stream=True) as r:
        r.raise_for_status()
        with open(local_filename, 'wb') as f:
            for chunk in r.iter_content(chunk_size=8192): 
                # If you have chunk encoded response uncomment if
                # and set chunk_size parameter to None.
                #if chunk: 
                f.write(chunk)
    return local_filename

tmp_dir = 'dist/'
out_dir = 'extracted/'
if os.path.isdir(tmp_dir):
    shutil.rmtree(tmp_dir)
if os.path.isdir(out_dir):
    shutil.rmtree(out_dir)
os.mkdir(tmp_dir)
os.mkdir(out_dir)

access_token = os.getenv('GITHUB_TOKEN', None)
if access_token is None:
    raise ValueError('No GITHUB_TOKEN found in environment variables')

g = GitHub(token=access_token)

run_id = int(input("Run ID: "))
artifacts = g.repos.arihant2math["prontus"].actions.runs[run_id].artifacts.get()[1]
for artifact in artifacts["artifacts"]:
    if ".app" not in artifact["name"]:
        #/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}
        response = g.repos.arihant2math["prontus"].actions.artifacts[artifact["id"]].zip.get()
        headers = g.getheaders()
        download_url = None
        for header in headers:
            if header[0] == "Location":
                download_url = header[1]
        file = str(Path(tmp_dir) / (artifact["name"] + ".zip"))
        print(f"Downloading {artifact["name"]} to {file}")
        download_file(download_url, file)
    else:
        print("Skipping " + artifact["name"])

# Gets all the .zip files in the temp directory
zip_files = [f for f in os.listdir(tmp_dir) if f.endswith('.zip')]

# Extracts all the .zip files in the output directory
for zip_file in zip_files:
    with zipfile.ZipFile(tmp_dir + zip_file) as zip_ref:
        # Extracts all, but we only expect there to be 1 file
        zip_ref.extractall(out_dir)
        print(f'Extracted {zip_file} to {out_dir}')
