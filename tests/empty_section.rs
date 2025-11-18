use sfa::{Reader, Writer};
use std::{
    fs::File,
    io::{Read, Write},
};

#[test]
pub fn empty_section() -> Result<(), sfa::Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("cherry_pie");

    let mut file = File::create(&path)?;
    let mut writer = Writer::from_writer(&mut file);
    writer.start("Verse 1")?;
    writer.write_all(b"Glazed eyes and cherry pie\n")?;
    writer.start("Verse 2")?;
    writer.start("Chorus 2")?;
    writer.write_all(b"Youth is running out, we finally feel it now\n")?;
    writer.finish()?;
    file.sync_all()?;
    drop(file);

    let reader = Reader::new(&path)?;
    {
        let toc = reader.toc();
        assert_eq!(3, toc.len());
        assert!(!toc.is_empty());

        assert_eq!(b"Verse 1", toc[0].name());
        let bytes = toc[0]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"Glazed eyes and cherry pie\n");

        assert_eq!(b"Verse 2", toc[1].name());
        let bytes = toc[1]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;

        assert_eq!(bytes, b"");

        assert_eq!(b"Chorus 2", toc[2].name());
        let bytes = toc[2]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"Youth is running out, we finally feel it now\n");
    }

    Ok(())
}

#[test]
pub fn empty_section_name() -> Result<(), sfa::Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("cherry_pie");

    let mut file = File::create(&path)?;
    let mut writer = Writer::from_writer(&mut file);
    writer.start("Verse 1")?;
    writer.write_all(b"Glazed eyes and cherry pie\n")?;
    writer.start("Verse 2")?;
    writer.start("")?;
    writer.write_all(b"Youth is running out, we finally feel it now\n")?;
    writer.finish()?;
    file.sync_all()?;
    drop(file);

    let reader = Reader::new(&path)?;
    {
        let toc = reader.toc();
        assert_eq!(3, toc.len());
        assert!(!toc.is_empty());

        assert_eq!(b"Verse 1", toc[0].name());
        let bytes = toc[0]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"Glazed eyes and cherry pie\n");

        assert_eq!(b"Verse 2", toc[1].name());
        let bytes = toc[1]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;

        assert_eq!(bytes, b"");

        assert_eq!(b"", toc[2].name());
        let bytes = toc[2]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"Youth is running out, we finally feel it now\n");
    }

    Ok(())
}
