-- Insert top-level categories
insert into category (id, ap_id, local, name, image_url, parent_id, sub_categories)
values 
('1', 'http://localhost/category/cat1', true, 'Electronics', 'https://example.com/electronics.jpg', null, '{"http://localhost/category/subcat1", "http://localhost/category/subcat2", "http://localhost/category/subcat3"}'),
('2', 'http://localhost/category/cat2', true, 'Clothing', 'https://example.com/clothing.jpg', null, '{"http://localhost/category/subcat4", "http://localhost/category/subcat5", "http://localhost/category/subcat6"}'),
('3', 'http://localhost/category/cat3', false, 'Books', null, null, '{"http://localhost/category/subcat7", "http://localhost/category/subcat8"}');

-- Insert sub-categories under Electronics
insert into category (id, ap_id, local, name, sub_categories, parent_id)
values
('4', 'http://localhost/category/subcat1', true, 'Mobile Phones', '{"http://localhost/category/item1", "http://localhost/category/item2"}', 'http://localhost/category/cat1'),
('5', 'http://localhost/category/subcat2', true, 'Laptops', '{"http://localhost/category/item3", "http://localhost/category/item4"}', 'http://localhost/category/cat1'),
('6', 'http://localhost/category/subcat3', true, 'Televisions', '{"http://localhost/category/item5", "http://localhost/category/item6"}', 'http://localhost/category/cat1');

-- Insert sub-categories under Clothing
insert into category (id, ap_id, local, name, sub_categories, parent_id)
values
('7', 'http://localhost/category/subcat4', true, 'T-Shirts', '{"http://localhost/category/item7", "http://localhost/category/item8"}', 'http://localhost/category/cat2'),
('8', 'http://localhost/category/subcat5', true, 'Jeans', '{"http://localhost/category/item9", "http://localhost/category/item10"}', 'http://localhost/category/cat2'),
('9', 'http://localhost/category/subcat6', true, 'Shoes', '{"http://localhost/category/item11", "http://localhost/category/item12"}', 'http://localhost/category/cat2');

-- Insert categories under Books
insert into category (id, ap_id, local, name, sub_categories, parent_id)
values
('10', 'http://localhost/category/subcat7', false, 'Fiction', '{"http://localhost/category/item13", "http://localhost/category/item14"}', 'http://localhost/category/cat3'),
('11', 'http://localhost/category/subcat8', false, 'Non-Fiction', '{"http://localhost/category/item15", "http://localhost/category/item16"}', 'http://localhost/category/cat3');

-- Insert specific items or deeper sub-categories
insert into category (id, ap_id, local, name, parent_id)
values
('12', 'http://localhost/category/item1', true, 'Smartphones', 'http://localhost/category/subcat1'),
('13', 'http://localhost/category/item2', true, 'Gaming Laptops', 'http://localhost/category/subcat2'),
('14', 'http://localhost/category/item3', true, '4K TVs', 'http://localhost/category/subcat3'),
('15', 'http://localhost/category/item4', true, 'Nike T-Shirts', 'http://localhost/category/subcat4'),
('16', 'http://localhost/category/item5', true, 'Levi Jeans', 'http://localhost/category/subcat5'),
('17', 'http://localhost/category/item6', false, 'Mystery Novels', 'http://localhost/category/subcat7'),
('18', 'http://localhost/category/item7', false, 'Biographies', 'http://localhost/category/subcat8');

