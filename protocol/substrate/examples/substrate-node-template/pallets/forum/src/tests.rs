use crate::mock::*;
use crate::Comment;
use crate::Post;
use codec::MaxEncodedLen;
use frame_support::assert_ok;
use frame_support::BoundedVec;

#[test]
fn it_works_posting_content() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let content = BoundedVec::try_from(b"hello".to_vec()).unwrap();
		let current_item = ForumModule::item_counter();
		assert_eq!(current_item, 0);
		assert_ok!(ForumModule::post_content(Origin::signed(1000), content.clone()));
		// Read pallet storage and assert an expected result.
		println!("post: {:#?}", ForumModule::post(0));
		assert_eq!(current_item, 0);

		assert_eq!(
			ForumModule::get_post(0),
			Some(Post {
				post_id: 0,
				content,
				author: 1000,
				timestamp: ForumModule::timestamp(),
				block_number: ForumModule::block_number(),
			})
		);

		let comment1 = BoundedVec::try_from(b"I'm 1st comment".to_vec()).unwrap();

		let item1 = ForumModule::item_counter();
		assert_ok!(ForumModule::comment_on(Origin::signed(2000), 0, comment1.clone()));
		assert_eq!(item1, 1);

		let item2 = ForumModule::item_counter();
		assert_ok!(ForumModule::comment_on(
			Origin::signed(2000),
			0,
			BoundedVec::try_from(b"This is a 2nd comment".to_vec()).unwrap()
		));

		assert_eq!(item2, 2);

		let item3 = ForumModule::item_counter();
		assert_ok!(ForumModule::comment_on(
			Origin::signed(3000),
			1,
			BoundedVec::try_from(b"> I'm a comment  to the 1st comment \nThis".to_vec()).unwrap()
		));
		assert_eq!(item3, 3);

		let item4 = ForumModule::item_counter();
		assert_ok!(ForumModule::comment_on(
			Origin::signed(3000),
			2,
			BoundedVec::try_from(b"I'm a comment for the 2nd comment".to_vec()).unwrap()
		));
		assert_eq!(item4, 4);

		assert_eq!(
			ForumModule::comment(1),
			Some(Comment {
				comment_id: 1,
				content: comment1,
				author: 2000,
				parent_item: 0,
				timestamp: ForumModule::timestamp(),
				block_number: ForumModule::block_number(),
			})
		);
		assert_eq!(ForumModule::kids(0), Some(BoundedVec::try_from(vec![1, 2]).unwrap()));
		assert_eq!(ForumModule::kids(1), Some(BoundedVec::try_from(vec![3]).unwrap()));
		assert_eq!(ForumModule::kids(2), Some(BoundedVec::try_from(vec![4]).unwrap()));

		assert_eq!(Post::<Test>::max_encoded_len(), 310);
		assert_eq!(Comment::<Test>::max_encoded_len(), 314);
	});
}
