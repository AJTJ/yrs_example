use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{Doc, StateVector, Update};

const TEXT_NAME: &str = "root-type-name";

fn main() {
    most_useful_example();
}

fn most_useful_example() {
    // a doc in the record
    let record_doc = Doc::new();
    let mut record_txn = record_doc.transact();
    let record_root_text = record_txn.get_text(TEXT_NAME);
    // add some text to the content, as if it were in previous existence
    record_root_text.push(&mut record_txn, "hello world");

    // bob's local doc
    let bob_doc = Doc::new();
    let mut bob_txn = bob_doc.transact();

    // HOW TO UPDATE BOB FROM THE RECORD
    // create state vector for bob, locally
    let bob_vector = bob_txn.state_vector().encode_v1();

    // create an update Vec that updates bob with what is held in the record_doc
    let update_to_bob = record_txn.encode_diff_v1(&StateVector::decode_v1(&bob_vector).unwrap());

    // apply this udpate to bob's txn to ensure that bob is working with the latest doc
    bob_txn.apply_update(Update::decode_v1(update_to_bob.as_slice()).unwrap());
    let bob_root = bob_txn.get_text(TEXT_NAME);
    assert!(bob_root.to_string() == record_root_text.to_string());

    // HOW TO UPDATE THE RECORD FROM BOB
    bob_root.insert(&mut bob_txn, 6, "beautiful ");
    println!("bob changes: {}", bob_root.to_string());
    assert_ne!(bob_root.to_string(), record_root_text.to_string());

    let record_vector = record_txn.state_vector().encode_v1();
    let update_to_record = bob_txn.encode_diff_v1(&StateVector::decode_v1(&record_vector).unwrap());
    record_txn.apply_update(Update::decode_v1(update_to_record.as_slice()).unwrap());
    assert!(bob_root.to_string() == record_root_text.to_string());

    // THIS SHOULD BE FAIRLY STRAIGHTFORWARD WITH ANY OTHER USERS
}

fn _basic_example() {
    let first_doc = Doc::new();
    let mut first_txn = first_doc.transact();
    let first_root_text = first_txn.get_text(TEXT_NAME);
    first_root_text.push(&mut first_txn, "hello world");
    println!("FIRST: {}", first_root_text.to_string());

    let second_doc = Doc::new();
    let mut second_txn = second_doc.transact();
    let second_root_text = second_txn.get_text(TEXT_NAME);
    second_root_text.push(&mut second_txn, ", friends!");
    println!("SECOND: {}", second_root_text.to_string());

    // in order to exchange data with other documents we first need to create a state vector
    let state_vector = second_txn.state_vector().encode_v1();

    // now compute a differential update based on the second document's state vector
    let update = first_txn.encode_diff_v1(&StateVector::decode_v1(&state_vector).unwrap());

    // now apply that differential to this second document
    second_txn.apply_update(Update::decode_v1(update.as_slice()).unwrap());

    println!(
        "SECOND AFTER UPDATE: {}",
        second_txn.get_text(TEXT_NAME).to_string()
    );
}

fn _singular_doc_test_1() {
    //apply different types of transactions to a singular doc

    let property_id: &str = "123";
    let root_doc = Doc::new();
    let mut bob_txn = root_doc.transact();
    let mut lucy_txn = root_doc.transact();
    let bob_root = bob_txn.get_text(property_id);
    let lucy_root = lucy_txn.get_text(property_id);

    lucy_root.push(&mut lucy_txn, "I am lucy");
    println!("lucy: {}", lucy_root.to_string());

    bob_root.push(&mut bob_txn, "I am bob");
    println!("bob: {}", bob_root.to_string());
}
