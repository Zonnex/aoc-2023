def create_dot_file(filename, output_filename):
    with open(output_filename, 'w') as f:
        f.write('digraph G {\n')
        with open(filename, 'r') as data_file:
            for line in data_file:
                nodes = line.strip().split()
                for node in nodes[1:]:
                    f.write('    {} -> {};\n'.format(nodes[0], node))
        f.write('}\n')

# Call the function with your input file name and desired output file name
create_dot_file('real.txt', 'graph.dot')