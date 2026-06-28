impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort();

        let mut rooms: Vec<Vec<(i32,i32)>> = Vec::with_capacity(n as usize);

        for meet in meetings.iter(){
            let mut start = meet[0];
            let mut end = meet[1];

            // First check if any existing room is free
            let mut free_room: Option<usize> = None;
            for (i, room) in rooms.iter().enumerate() {
                if let Some(&(_, e)) = room.last() {
                    if start >= e {
                        free_room = Some(i);
                        break;
                    }
                }
            }

            if let Some(i) = free_room {
                rooms[i].push((start, end));
            } else if rooms.len() == n as usize {
                let mut min = rooms[0].last().unwrap().1;
                let mut mini = 0;

                for (i, room) in rooms.iter().enumerate() {
                    if let Some(&(_, e)) = room.last() {
                        if e < min || (e == min && i < mini) {
                            mini = i;
                            min = e;
                        }
                    }
                }

                let &(_, e) = rooms[mini].last().unwrap();
                end = e + end - start;
                start = e;
                rooms[mini].push((start, end));
            } else {
                rooms.push(vec![(start, end)]);
            }
        }

        let mut maxlen = 0;
        let mut index = 0;
        for (i, room) in rooms.iter().enumerate(){
            if maxlen < room.len(){
                maxlen = room.len();
                index = i;
            }
        }

        index as i32
    }
}