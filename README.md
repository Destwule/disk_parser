# disk_parser

A small command-line tool, written in Rust, that reads the partition table of a disk or disk image and prints it as a table, similar to `fdisk -l`.

It supports both **MBR** and **GPT**. It tries GPT first and falls back to MBR if no GPT header is found.

## Usage

```
cargo run -- <disk_path>
```

`disk_path` can be a disk image file or a device such as `/dev/sdb` (reading a real device usually needs `sudo`). The tool only reads and never writes.

## Example

```
Device     Start End    Sectors Size Type
gpt.img1   2048  104447 102400  50M  Linux filesystem
```

## Try it without touching a real disk

```
truncate -s 100M test.img
printf 'label: gpt\nstart=2048, size=50M, type=L\n' | sfdisk test.img
cargo run -- test.img
```

Use `label: dos` instead of `label: gpt` to make an MBR image.

## Limitations

- Assumes 512-byte sectors
- Does not verify GPT checksums
- Does not follow MBR extended partitions

## Notes

A learning project for working with binary disk formats in Rust.