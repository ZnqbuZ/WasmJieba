import os
import shutil
import pypinyin

import jieba

jieba.enable_paddle()
# jieba.enable_parallel(12)

print("Loading user dict...")
jieba.load_userdict(r".\dict.txt.big")

print("Building ime_dicts...")
shutil.copy(r"../../assets/dict.txt.big", r".\out\dict.txt.big")
for dict in os.listdir("../../assets/ime_dicts"):
    with open(f"../../assets/ime_dicts/{dict}", "r", encoding="utf-8") as fin:
        print(dict)
        new_dict = set([])
        for line in fin:
            for word in jieba.cut(line.strip(), use_paddle=True, HMM=False, cut_all=True):
                if len(word) > 1:
                    new_dict.add(word)
    with open(f"out/{dict}", "w", encoding="utf-8") as fout:
        new_dict = sorted(new_dict, key=pypinyin.lazy_pinyin)
        for word in new_dict:
            fout.write(word + "\n")
