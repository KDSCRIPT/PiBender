from hciutils import RustList, RustMap, RustSet

def main():
    print("\n--- RustList Demo ---")
    rust_list = RustList()
    print("Initial list:", rust_list.get_data())
    
    # Append values
    rust_list.append(10)
    rust_list.append(20)
    print("After appending 10 and 20:", rust_list.get_data())
    
    # Insert value
    rust_list.insert(1, 15)
    print("After inserting 15 at index 1:", rust_list.get_data())
    
    # Remove value
    rust_list.remove(10)
    print("After removing 10:", rust_list.get_data())
    
    # Pop value
    popped_value = rust_list.pop()
    print("Popped value:", popped_value)
    print("After popping:", rust_list.get_data())
    
    # Clear list
    rust_list.clear()
    print("After clearing:", rust_list.get_data())
    
    # Using other methods
    rust_list.append(5)
    rust_list.append(5)
    rust_list.append(10)
    print("List after adding duplicates:", rust_list.get_data())
    print("Index of 10:", rust_list.index(10))
    print("Count of 5:", rust_list.count(5))
    rust_list.reverse()
    print("After reversing:", rust_list.get_data())
    rust_list.sort()
    print("After sorting:", rust_list.get_data())
    
    print("\n--- RustMap Demo ---")
    rust_map = RustMap()
    rust_map.insert(1, 100)
    rust_map.insert(2, 200)
    print("Map after inserting (1: 100) and (2: 200):", rust_map.to_dict())
    
    # Get value
    value = rust_map.get(1)
    print("Value for key 1:", value)
    
    # Check if key exists
    print("Contains key 2:", rust_map.contains_key(2))
    
    # Remove key
    rust_map.remove(1)
    print("Map after removing key 1:", rust_map.to_dict())
    
    # Clear map
    rust_map.clear()
    print("Map after clearing:", rust_map.to_dict())
    
    print("\n--- RustSet Demo ---")
    rust_set = RustSet()
    rust_set.add(1)
    rust_set.add(2)
    rust_set.add(3)
    print("Set after adding 1, 2, 3:", rust_set.to_list())
    
    # Check if value exists
    print("Contains 2:", rust_set.contains(2))
    
    # Remove a value
    rust_set.remove(2)
    print("Set after removing 2:", rust_set.to_list())
    
    # Clear set
    rust_set.clear()
    print("Set after clearing:", rust_set.to_list())

if __name__ == "__main__":
    main()
