# bighash

Hash a big file using a Merkle Tree.

# Motivation

Recently a RAID1 drive at my parents' place was marked as failed by mdadm. I've retrieved the failed drive (and the data within), and I want to replicate the data on the store at my place. Problem is as follows:

- I don't know the extent (if any) of the corruption on the drive I've retrieved
- I'm not keen on spending months trying to copy data over the internet.
- I don't have another free storage medium available for transferring data sneakernet style. This "failed" RAID1 drive is all I got to work with.
- I won't be excited to find a massive 3GB file corrupted and having to retrieve that over the air over the course of hours.

This is solution I've come up with: by splitting the files I want to verify into manageable chunks, I can identify the subsections of the file which are corrupted, and only retrieve those OTA.
