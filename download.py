from huggingface_hub import hf_hub_download

for i in range(1, 11):
    path = "de_meta/de_meta_part_{}.jsonl.zst".format(i)
    hf_hub_download(repo_id="oscar-corpus/OSCAR-2301", filename=path, repo_type="dataset", local_dir="oscar-test", local_dir_use_symlinks=False)