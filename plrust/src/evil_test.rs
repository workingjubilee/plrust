#[pg_test]
#[search_path(@extschema@)]
// #[should_panic(expected = "error: declaration of a function with `export_name`")]
fn plrust_block_unsafe_link() -> spi::Result<()> {
    let definition = r#"
        CREATE OR REPLACE FUNCTION links_bad_lib() RETURNS BIGINT
        IMMUTABLE STRICT
        LANGUAGE PLRUST AS
        $$
            // Maybe we already have a latent library on the system somehow?
            #[link(name = "UnsafeLibrary", kind = "dylib")]
            extern {

            }

            Ok(Some(1))
        $$;
    "#;
    Spi::run(definition)?;
    let result = Spi::get_one::<i32>("SELECT links_bad_lib();\n");
    assert_eq!(Ok(Some(1)), result);
    Ok(())
}