use crate::virtual_branches::list;
use gitbutler_branch_actions::BranchListingDetails;

#[test]
fn one_vbranch_on_integration_empty_details() -> anyhow::Result<()> {
    let list = branch_details(
        &list::project_ctx("one-vbranch-on-integration")?,
        Some("virtual"),
    )?;
    assert_eq!(list.len(), 1);
    assert_eq!(
        list[0],
        BranchListingDetails {
            name: "virtual".into(),
            lines_added: 0,
            lines_removed: 0,
            number_of_files: 0,
            number_of_commits: 0,
            authors: vec![],
        }
    );
    Ok(())
}

#[test]
fn one_vbranch_on_integration_single_commit() -> anyhow::Result<()> {
    let list = branch_details(
        &list::project_ctx("one-vbranch-on-integration-one-commit")?,
        Some("virtual"),
    )?;
    assert_eq!(list.len(), 1);
    assert_eq!(
        list[0],
        BranchListingDetails {
            name: "virtual".into(),
            lines_added: 2,
            lines_removed: 0,
            number_of_files: 2,
            number_of_commits: 1,
            authors: vec![default_author()],
        }
    );
    Ok(())
}

#[test]
fn many_commits_in_all_branch_types() -> anyhow::Result<()> {
    let ctx = project_ctx("complex-repo")?;
    let list = branch_details(&ctx, ["feature", "main", "non-virtual-feature"])?;
    assert_eq!(list.len(), 3);
    assert_eq!(
        list[0],
        BranchListingDetails {
            name: "feature".into(),
            lines_added: 100 + 5,
            lines_removed: 0,
            number_of_files: 1,
            number_of_commits: 100 + 5, /* local tracking branch is merge base */
            authors: vec![default_author()],
        }
    );
    assert_eq!(
        list[1],
        BranchListingDetails {
            name: "main".into(),
            lines_added: 15,
            lines_removed: 0,
            number_of_files: 1,
            // TODO(ST): why is it also going against the local tracking branch instead of the local `main`?
            number_of_commits: 10 + 5,
            authors: vec![default_author()],
        }
    );
    assert_eq!(
        list[2],
        BranchListingDetails {
            name: "non-virtual-feature".into(),
            lines_added: 55,
            lines_removed: 0,
            number_of_files: 1,
            number_of_commits: 50 + 5,
            authors: vec![default_author()],
        }
    );
    Ok(())
}

mod util {
    use gitbutler_branch::BranchIdentity;
    use gitbutler_branch_actions::{Author, BranchListingDetails};
    use gitbutler_command_context::CommandContext;

    pub fn branch_details(
        ctx: &CommandContext,
        branch_names: impl IntoIterator<Item = impl TryInto<BranchIdentity>>,
    ) -> anyhow::Result<Vec<BranchListingDetails>> {
        let mut details = gitbutler_branch_actions::get_branch_listing_details(ctx, branch_names)?;
        details.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(details)
    }

    pub fn default_author() -> Author {
        Author {
            name: Some("author".into()),
            email: Some("author@example.com".into()),
        }
    }

    pub fn project_ctx(name: &str) -> anyhow::Result<CommandContext> {
        gitbutler_testsupport::read_only::fixture("for-details.sh", name)
    }
}
use util::{branch_details, default_author, project_ctx};
