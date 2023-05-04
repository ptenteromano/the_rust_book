use collections_ch8::companies;

fn main() {
    // Hashmap - Company organization
    let mut company = companies::Company::new();

    company.adjust("add maddie to marketing");
    company.adjust("add phil to engineering");
    company.adjust("add bob to marketing");
    company.list_all();

    company.adjust("remove bob from marketing");
    company.list("marketing");
}
