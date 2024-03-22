trait Commutativity<T, O> {

}

trait Associativity<T, O> {

}

trait LeftDistrubutivity<T, O> {

}

trait RightDistrubutivity<T, O> {
    
}

trait Distrubutivity<T, O>: LeftDistrubutivity<T, O> + RightDistrubutivity<T, O> {
    
}

// trait AutoDistributivity = Distributivity;