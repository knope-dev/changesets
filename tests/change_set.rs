use changesets::{BumpType, ChangeSet, PackageChange};
use tempfile::tempdir;

#[test]
fn load_changeset() {
    let dir = tempdir().unwrap();

    let first_change_name = "first_change";
    let first_change_path = dir.path().join(format!("{first_change_name}.md"));
    let first_package = "first_package";
    let second_package = "second_package";
    let first_change_bump = BumpType::Minor;
    let second_package_bump = BumpType::Patch;
    let first_change_summary = "### This is a summary";
    std::fs::write(
        &first_change_path,
        format!(
            "---\n{first_package}: {first_change_bump}\n{second_package}: {second_package_bump}\n---\n\n{first_change_summary}\n",
        ),
    )
    .unwrap();

    let second_change_name = "second_change";
    let second_change_path = dir.path().join(format!("{second_change_name}.md"));
    let second_change_bump = BumpType::Major;
    let second_change_summary = "### Another summary";
    std::fs::write(
        &second_change_path,
        format!("---\n{second_package}: {second_change_bump}\n---\n\n{second_change_summary}\n",),
    )
    .unwrap();

    let changeset = ChangeSet::from_directory(&dir).unwrap();
    let first_release = changeset.releases.get(first_package).unwrap();
    assert_eq!(first_release.package_name, first_package);
    assert_eq!(first_release.bump_type(), first_change_bump);
    assert_eq!(
        first_release.changes,
        vec![PackageChange {
            bump_type: first_change_bump,
            summary: first_change_summary.to_string()
        },]
    );
    let second_release = changeset.releases.get(second_package).unwrap();
    assert_eq!(second_release.package_name, second_package);
    assert_eq!(second_release.bump_type(), second_change_bump);
    // Order of reading files is probably not guaranteed
    let first_variant = vec![
        PackageChange {
            bump_type: second_package_bump,
            summary: first_change_summary.to_string(),
        },
        PackageChange {
            bump_type: second_change_bump,
            summary: second_change_summary.to_string(),
        },
    ];
    let second_variant = first_variant.iter().cloned().rev().collect::<Vec<_>>();
    assert!(
        second_release.changes == first_variant || second_release.changes == second_variant,
        "Expected {:?} or {:?}, got {:?}",
        first_variant,
        second_variant,
        second_release.changes
    );
}
